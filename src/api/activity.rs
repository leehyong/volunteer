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
        query
            .push_sql(LIMIT_NUM_SQL)
            .check()
            .map_or_else(|e| {
                Vec::new()
            }, |w| block_on(async move {
                DB.list_by_wrapper::<Activity>("", &w)
                    .await
                    .map_or_else(|e| {
                        Vec::new()
                    }, |v| v)
            }))
    }

    async fn info(id: u32) -> Option<Activity> {
        let mut query = DB.new_wrapper();
        query
            .eq("id", id)
            .eq("is_delete", 0);
        query.check().map_or_else(|e| {
            None
        }, |w| block_on(async move {
            DB.fetch_by_wrapper::<Activity>("", &w)
                .await
                .map_or_else(|e| {
                    None
                }, |v| Some(v))
        }),
        )
    }

    pub async fn detail(req: Request<AppState>) -> TideResult {
        req.param("id")
            .map_or_else(|e| ResponseUtil::error(e.to_string()),
                         |param| {
                             param.parse::<u32>()
                                 .map_or_else(
                                     |e| ResponseUtil::error(e.to_string()),
                                     |n| block_on(async move {
                                         ResponseUtil::ok(Self::info(n).await)
                                     }))
                         }
            )
    }

    pub async fn new(req: Request<AppState>) -> TideResult {
        Ok("new".into())
    }
}
