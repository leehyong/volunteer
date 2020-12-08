use crate::import::*;
use crate::AppState;
use crate::model::Activity;
use crate::util::ResponseUtil;
use crate::util::{ActivityReq, Subject, ActivityType};
use validator::Validate;
use rbatis::crud::CRUD;
use rbatis::core::value::DateTimeNow;


pub struct ActivityApi;

impl ActivityApi {
    pub async fn list(req: Request<AppState>) -> TideResult {
        let params = req.query::<ActivityReq>()?;
        // info!("params, {:?}", &params);
        match params.validate() {
            Ok(_) => {
                ResponseUtil::ok(Self::select(
                    &params.end_time,
                    params.start_time.as_ref(),
                    params.activity_types,
                    params.subjects,
                ).await)
            }
            Err(e) => {
                ResponseUtil::error(e.to_string())
            }
        }
    }

    async fn select(end: &NaiveDateTime, start: Option<&NaiveDateTime>, types: Option<Vec<ActivityType>>, subjects: Option<Vec<Subject>>) -> Vec<Activity> {
        let mut query = DB.new_wrapper();
        let mut query = query
            .eq("is_delete", 0)
            .gt("end_time", *end);
        if start.is_some() {
            query.gt("start_time", *start.unwrap());
        }
        if types.is_some() {
            let types = types.unwrap();
            let types = types.iter().map(|d| d.at.as_str()).collect::<Vec<&str>>();
            query.r#in("activity_type", &types);
        }
        if subjects.is_some() {
            let  subjects = subjects.unwrap();
            let subjects = subjects.iter().map(|d| d.sub.as_str()).collect::<Vec<&str>>();
            query.r#in("subject", &subjects);
        }
        match query.push_sql(LIMIT_NUM_SQL).check() {
            Ok(w) => {
                let r: DbResult<Vec<Activity>> = DB.list_by_wrapper("", &w).await;
                match r {
                    Ok(d) => {
                        return d;
                    }
                    Err(e) => {
                        error!("{}", e.to_string());
                        return vec![];
                    }
                }
            }
            Err(e) => {
                error!("{}", e.to_string());
                return vec![];
            }
        }
    }

    async fn info(id: i32) -> Option<Activity> {
        let mut query = DB.new_wrapper();
        let mut query = query
            .eq("id", id)
            .eq("is_delete", 0);
        match query.check() {
            Ok(w) => {
                let r: DbResult<Activity> = DB.fetch_by_wrapper("", &w).await;
                match r {
                    Ok(d) => {
                        return Some(d);
                    }
                    Err(e) => {
                        error!("{}", e.to_string());
                        return None;
                    }
                }
            }
            Err(e) => {
                error!("{}", e.to_string());
                return None;
            }
        }
    }

    pub async fn detail(mut req: Request<AppState>) -> TideResult {
        ResponseUtil::ok(Self::info(1).await)
    }

    pub async fn new(mut req: Request<AppState>) -> TideResult {
        Ok("new".into())
    }
}
