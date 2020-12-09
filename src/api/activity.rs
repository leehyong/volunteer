use crate::import::*;
use crate::AppState;
use crate::model::Activity;
use crate::util::ResponseUtil;
use crate::util::{ActivityReq};
use validator::Validate;
use rbatis::crud::CRUD;
use rbatis::core::value::DateTimeNow;


pub struct ActivityApi;

impl ActivityApi {
    pub async fn list(req: Request<AppState>) -> TideResult {
        let req = req.query::<ActivityReq>()?;
        match req.validate() {
            Ok(_) => {
                ResponseUtil::ok(Self::select(&req).await)
            }
            Err(e) => {
                ResponseUtil::error(e.to_string())
            }
        }
    }

    async fn select(req: &ActivityReq) -> Vec<Activity> {
        let mut query = DB.new_wrapper();
        query
            .eq("is_delete", 0)
            .lt("end_time", req.end_time);
        if req.start_time.is_some() {
            query.gt("start_time", req.start_time.as_ref().unwrap());
        }
        if req.activity_type.is_some() {
            query.eq("activity_type", req.activity_type.as_ref().unwrap().as_str());
        }
        if req.subject.is_some() {
            query.eq("subject", req.subject.as_ref().unwrap().as_str());
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

    async fn info(id: u32) -> Option<Activity> {
        let mut query = DB.new_wrapper();
        query
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

    pub async fn detail(req: Request<AppState>) -> TideResult {
        let id_req = req.param("id")?;
        match id_req.parse::<u32>() {
            Ok(n) => {
                ResponseUtil::ok(Self::info(n).await)
            }
            Err(e) => {
                let s = format!("{}: {}", e.to_string(), id_req);
                error!("{}", &s);
                ResponseUtil::error(s)
            }
        }
    }

    pub async fn new(mut req: Request<AppState>) -> TideResult {
        Ok("new".into())
    }
}
