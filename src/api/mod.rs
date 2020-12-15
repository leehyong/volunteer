mod activity;
mod user;

use crate::import::*;
use crate::api::activity::{ActivityApi, ApplyApi};
use crate::api::user::UserApi;
use crate::jwt::jwt_auth_middleware;
use crate::state::AppState;
use tide::{with_state, Server};

pub fn api_route(app: &mut Server<AppState>) {
    api_auth_route(app);
    api_no_auth_route(app);
    admin_auth_route(app);
}

fn api_auth_route(app: &mut Server<AppState>) {
    let app_state = app.state().clone();
    // 这里加入需要鉴权和认证的的URL
    app.at(API_PATH)
        // 自动在路径前面加上 '/'
        .with(jwt_auth_middleware)
        .nest(
            {
                let mut api = tide::with_state(app_state);
                api.at("apply/:activity_id")
                    .post(ApplyApi::post)
                    .delete(ApplyApi::delete);
                api
            }
        );
}


fn api_no_auth_route(app: &mut Server<AppState>) {
    // 这里加入不需要鉴权和认证的的URL
    let app_state = app.state().clone();
    app.at(API_PATH)
        .nest({
            let mut api = tide::with_state(app_state);
            api
                .at("activity")
                .get(ActivityApi::list);
            api
                .at("activity/:id")
                .get(ActivityApi::detail);
            api
                .at("login")
                .post(UserApi::login);
            api
        });
}

fn admin_auth_route(app: &mut Server<AppState>) {
    // 这里加入管理后台需要鉴权和认证的的URL
    let app_state = app.state().clone();
    app.at(ADMIN_PATH)
        .with(jwt_auth_middleware)
        .nest({
            let mut admin = tide::with_state(app_state);
            admin
                .at("users")
                .get(|_| async { Ok("all users") });
            admin
                .at("activity")
                .post(ActivityApi::post)
                .at(":id")
                .put(ActivityApi::put);
            admin
        });
}