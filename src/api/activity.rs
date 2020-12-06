use crate::import::*;
use crate::AppState;
use crate::model::Activity;
use crate::util::ResponseUtil;
use rbatis::crud::CRUD;
use rbatis::core::value::DateTimeNow;


pub struct ActivityApi;

impl ActivityApi {
    pub async fn list(mut req: Request<AppState>) -> TideResult {
        ResponseUtil::ok(Self::select(&NaiveDateTime::now(), None, None, None).await)
    }

    async fn select(end: &NaiveDateTime, start: Option<&NaiveDateTime>, types: Option<&Vec<String>>, subjects: Option<&Vec<String>>) -> Vec<Activity> {
        let mut query = DB.new_wrapper();
        let mut query = query
            .eq("is_delete", 0)
            .gt("end_time", *end);
        if start.is_some() {
            query.gt("start_time", *start.unwrap());
        }
        if types.is_some() {
            query.r#in("activity_type", types.unwrap());
        }
        if subjects.is_some() {
            query.r#in("subject", subjects.unwrap());
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
