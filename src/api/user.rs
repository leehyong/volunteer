use crate::import::*;
use crate::AppState;
use crate::model::User;
use crate::util::ResponseUtil;
use crate::util::datetime_util;
use async_std::sync::Arc;


pub struct UserApi;

impl UserApi {
    pub async fn login(req: Request<AppState>) -> TideResult {

        ResponseUtil::ok(())
    }
}