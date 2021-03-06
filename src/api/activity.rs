use async_std::sync::Arc;
use rbatis::core::db::DBExecResult;
use rbatis::crud::CRUD;

use crate::AppState;
use crate::import::*;
use crate::model::{Activity, User};
use crate::req::{ActivityReq, NewActivityReq, UpdateActivityReq};
use crate::util::datetime_util::*;
use crate::util::ResponseUtil;

pub struct ActivityApi;

impl ActivityApi {
    pub async fn list(req: Request<AppState>) -> TideResult {
        req.query::<ActivityReq>()
            .map_or_else(|e| {
                ResponseUtil::error(e.to_string())
            }, |v| {
                v.validate().map_or_else(|e| {
                    ResponseUtil::error(e)
                }, |_| block_on(async move {
                    ResponseUtil::ok(Self::select(&v).await)
                }))
            })
    }

    async fn select(req: &ActivityReq) -> Page<Activity> {
        let page = PageRequest::new(0, 50);
        let mut query = DB.new_wrapper();
        query
            .eq("is_delete", 0)
            .lt("end_time", req.end_date);
        if req.start_date.is_some() {
            query.gt("start_time", req.start_date.as_ref().unwrap());
        }
        if req.activity_type.is_some() {
            query.eq("activity_type", req.activity_type.as_ref().unwrap().as_str());
        }
        if req.subject.is_some() {
            query.eq("subject", req.subject.as_ref().unwrap().as_str());
        }
        query
            .check()
            .map_or_else(|e| {
                Page::new(0, 0)
            }, |w| block_on(async move {
                DB.fetch_page_by_wrapper("", &w, &page)
                    .await
                    .map_or_else(|e| {
                        error!("{}", e.to_string());
                        Page::new(0, 0)
                    }, |v| v)
            }))
    }


    pub async fn detail(req: Request<AppState>) -> TideResult {
        req.param("id")
            .map_or_else(|e| ResponseUtil::error(e.to_string()), |param| {
                param.parse::<u32>()
                    .map_or_else(
                        |e| ResponseUtil::error(e.to_string()),
                        |n| block_on(async move {
                            ResponseUtil::ok(Activity::info(n).await)
                        }))
            })
    }

    pub async fn post(mut ctx: Request<AppState>) -> TideResult {
        let user = ctx.ext::<Arc<User>>(); // todo ，获取真正的用户id
        let creator_id = user.unwrap().id;

        ctx.body_json::<NewActivityReq>().await
            .map_or_else(|e| ResponseUtil::error(e.to_string()), |req|
                {
                    match req.validate() {
                        Ok(_) => block_on(async move {
                            match DB.begin_tx().await {
                                Ok(txt_id) => {
                                    let sql = r#"
                                    insert into activity(
                                      creator_id, last_editor_id, subject, activity_type,
                                      apply_url, content, start_time, end_time) values (
                                      #{creator_id},#{last_editor_id},#{subject},#{activity_type},
                                      #{apply_url},#{creator_id},#{start_time}, #{end_time}
                                      );"#;
                                    match DB.py_exec(&txt_id, sql, &json!({
                                    "creator_id":creator_id,
                                    "last_editor_id":creator_id,
                                    "start_time":req.start_time,
                                    "end_time":req.end_time.unwrap_or(max_naive_datetime()),
                                    "subject":req.subject,
                                    "apply_url":req.apply_url,
                                    "activity_type":req.activity_type,
                                    "content":req.content,
                                })).await {
                                        Ok(result) => {
                                            match DB.commit(&txt_id).await {
                                                Ok(_) => ResponseUtil::ok(result),
                                                Err(e) => {
                                                    DB.rollback(&txt_id).await?;
                                                    ResponseUtil::error(e.to_string())
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            DB.rollback(&txt_id).await?;
                                            ResponseUtil::error(e.to_string())
                                        }
                                    }
                                }
                                Err(e) => {
                                    ResponseUtil::error(e.to_string())
                                }
                            }
                        }),
                        Err(e) => ResponseUtil::error(json!(e))
                    }
                },
            )
    }

    pub async fn put(mut ctx: Request<AppState>) -> TideResult {
        ctx.body_json::<UpdateActivityReq>().await
            .map_or_else(|e| ResponseUtil::error(e.to_string()), |req|
                match req.validate() {
                    Ok(_) =>
                        block_on(async move {
                            let id_str = ctx.param("id")?;
                            let _id = match id_str.parse::<u32>() {
                                Ok(id) => id,
                                Err(e) => return ResponseUtil::error(format!("{}:{}", id_str, e.to_string()))
                            };
                            let mut query = DB.new_wrapper();
                            query.eq("id", _id)
                                .eq("is_delete", 0);
                            let mut update = false;
                            let mut sets = Vec::with_capacity(16);
                            if req.end_time.is_some() {
                                update = true;
                                sets.push(format!("end_time = '{}'", req.end_time.unwrap()));
                            }
                            if req.start_time.is_some() {
                                update = true;
                                sets.push(format!("start_time = '{}'", req.start_time.unwrap()));
                            }
                            if req.activity_type.is_some() {
                                update = true;
                                sets.push(format!("activity_type = '{}'", req.activity_type.unwrap()));
                            }
                            if req.subject.is_some() {
                                update = true;
                                sets.push(format!("subject = '{}'", req.subject.unwrap()));
                            }
                            if req.apply_url.is_some() {
                                update = true;
                                sets.push(format!("apply_url = '{}'", req.apply_url.unwrap()));
                            }
                            if req.content.is_some() {
                                update = true;
                                sets.push(format!("content = '{}'", req.content.unwrap()));
                            }
                            if !update {
                                return ResponseUtil::ok(());
                            }
                            let creator_id = 1u32; // todo ，获取真正的用户id
                            sets.push(format!("last_editor_id ={}", creator_id));
                            let sql = format!("update activity set {} where id={} and is_delete=0", sets.join(","), _id);
                            match DB.begin_tx().await {
                                Ok(txt_id) => {
                                    match DB.exec(&txt_id, &sql).await {
                                        Ok(result) => {
                                            match DB.commit(&txt_id).await {
                                                Ok(d) => ResponseUtil::ok(json!({"id":_id})),
                                                Err(e) => {
                                                    DB.rollback(&txt_id).await?;
                                                    ResponseUtil::error(e.to_string())
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            DB.rollback(&txt_id).await?;
                                            ResponseUtil::error(e.to_string())
                                        }
                                    }
                                }
                                Err(e) => ResponseUtil::error(e.to_string())
                            }
                        }),
                    Err(e) =>
                        ResponseUtil::error(json!(e))
                },
            )
    }
}

pub struct ApplyApi;


impl ApplyApi {
    pub async fn delete(mut ctx: Request<AppState>) -> TideResult {
        // 取消活动申请
        Self::handle(ctx, true).await
    }
    pub async fn post(mut ctx: Request<AppState>) -> TideResult {
        // 申请活动报名
        Self::handle(ctx, false).await
    }

    async fn handle(mut ctx: Request<AppState>, is_cancel: bool) -> TideResult {
        let user = ctx.ext::<Arc<User>>();
        let user_id = user.unwrap().id;
        let activity_id;
        match ctx.param("activity_id")?.parse::<u32>() {
            Ok(aid) => {
                activity_id = aid;
            }
            Err(e) => return ResponseUtil::error(format!("{}:", e.to_string()))
        }
        return if let Some(act) = Activity::info(activity_id).await {
            let now = current_timestamp();
            if naive2timestamp(act.end_time) < now {
                return ResponseUtil::error(format!("{}:活动已经过期", activity_id));
            }
            match Self::apply_or_cancel(user_id, activity_id, is_cancel).await {
                Ok(d) => return ResponseUtil::ok(d),
                Err(e) => {
                    let e = e.to_string();
                    if e.contains(DUPLICATE_ENTRY) {
                        return ResponseUtil::ok(DBExecResult { rows_affected: 0, last_insert_id: None });
                    }
                    return ResponseUtil::error(e);
                }
            }
        } else {
            return ResponseUtil::error(format!("活动不存在:{}", activity_id));
        };
    }


    async fn apply_or_cancel(user_id: u32, activity_id: u32, is_cancel: bool) -> DbResult<DBExecResult> {
        let sql;
        let v = json!({"user_id":user_id, "activity_id":activity_id});
        if is_cancel {
            sql = r#"update apply set is_delete = 1
            where is_delete=0 and user_id=#{user_id} and activity_id=#{activity_id}"#;
        } else {
            sql = r#"
                insert into apply(user_id, activity_id)
                values( #{user_id}, #{activity_id})
                ON DUPLICATE KEY UPDATE is_delete=0"#;
        }
        DB.py_exec("", sql, &v)
            .await
    }
}