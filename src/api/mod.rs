mod activity;

use crate::api::activity::ActivityApi;
use crate::import::*;
use crate::jwt::jwt_auth_middleware;
use crate::state::AppState;
use tide::{with_state, Server};

pub fn api_route(app: &mut Server<AppState>) {
    // 自动在路径前面加上 '/'
    app.at(API_PATH)
        .at("activity")
        .get(ActivityApi::list)
        .post(ActivityApi::new)
        .at(":id")
        .get(ActivityApi::detail);
    app.at(API_PATH)
        .at("apply")
        .with(jwt_auth_middleware)
        .post(|_| async { Ok("Hello, world!") });
    app.at(ADMIN_PATH)
        .with(jwt_auth_middleware)
        .at("users")
        .get(|_| async { Ok("all users") });
}
