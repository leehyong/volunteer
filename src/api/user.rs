use crate::import::*;
use crate::AppState;
use crate::model::User;
use crate::util::ResponseUtil;
use crate::util::datetime_util;
use crate::req::UserReq;
use async_std::sync::Arc;


pub struct UserApi;

impl UserApi {
    // pub async fn login(mut req: Request<AppState>) -> TideResult {
    //     let err;
    //     match req.body_json::<UserReq>().await{
    //         Ok(ur) =>{
    //             match ur.validate() {
    //
    //             }
    //             ResponseUtil::ok(())
    //         }
    //         Err(e) => err = e
    //     }
    // }
}