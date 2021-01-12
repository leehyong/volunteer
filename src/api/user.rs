use crate::import::*;
use crate::AppState;
use crate::model::User;
use crate::util::ResponseUtil;
use crate::util::datetime_util;
use crate::req::UserReq;
use async_std::sync::Arc;
use crate::jwt::JwtClaims;


pub struct UserApi;

impl UserApi {
    pub async fn login(mut req: Request<AppState>) -> TideResult {
        let mut err_str = "".to_string();
        // todo 通过用户的账户密码进行登陆, 而不是用户id
        match req.body_json::<UserReq>().await {
            Ok(ur) => {
                match ur.validate() {
                    Ok(_) => {
                        let state = req.state();
                        match User::info(ur.user_id).await {
                            Ok(u) => {
                                let now = datetime_util::current_timestamp();
                                let arc_user = Arc::new(u);
                                state.set_user((now, arc_user.clone())).await;
                                return ResponseUtil::ok(json!({
                                    "token":JwtClaims::new(ur.user_id).gen_token()
                                }));
                            }
                            Err(e) => err_str = e.to_string()
                        }
                    }
                    Err(e) => return ResponseUtil::error(json!(e))
                }
            }
            Err(e) => err_str = e.to_string()
        }
        ResponseUtil::error(err_str)
    }

    pub async fn logout(mut ctx: Request<AppState>) -> TideResult {
        let user = ctx.ext::<Arc<User>>();
        let user_id = user.unwrap().id;
        ctx.state().remove_user(user_id).await;
        ResponseUtil::ok(json!({"status":"success"}))
    }
}