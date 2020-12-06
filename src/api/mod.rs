mod activity;

use crate::api::activity::ActivityApi;
use crate::import::*;
use crate::jwt::jwt_auth_middleware;
use crate::state::AppState;
use tide::{with_state, Server};

pub fn api_route(app: &mut Server<AppState>) {
    api_auth_route(app);
    api_no_auth_route(app);
    admin_auth_route(app);
}

fn api_auth_route(app: &mut Server<AppState>) {
    // 这里加入需要鉴权和认证的的URL
    app.at(API_PATH)
        // 自动在路径前面加上 '/'
        .with(jwt_auth_middleware)
        .at("apply")
        .post(|_| async { Ok("Hello, world!") });
}


fn api_no_auth_route(app: &mut Server<AppState>) {
    // 这里加入不需要鉴权和认证的的URL
    app.at(API_PATH)
        .at("activity")
        .get(ActivityApi::list)
        .at(":id")
        .get(ActivityApi::detail);
}

fn admin_auth_route(app: &mut Server<AppState>) {
    // 这里加入管理后台需要鉴权和认证的的URL
    app.at(ADMIN_PATH)
        .with(jwt_auth_middleware)
        // 自动在路径前面加上 '/'
        .at("users")
        .get(|_| async { Ok("all users") });
    app.at(ADMIN_PATH)
        .at("activity")
        .post(ActivityApi::new);
}