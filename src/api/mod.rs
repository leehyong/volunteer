mod activity;

use tide::Server;
use crate::import::*;
use crate::state::AppState;
use crate::jwt::JwtMiddleware;
use crate::api::activity::ActivityApi;

pub fn api_route(app: &mut Server<AppState>){
    app.register(
        root()
            .at("api/v1/", |route| {
                route
                    .at("activity", |route| {
                        route
                            .get(ActivityApi::list)
                            .post(ActivityApi::new)
                            .at("/{}", |route| {
                                route.get(ActivityApi::detail)
                            })
                    })
                    .with(JwtMiddleware, |route|{
                        route.at("/apply2", |route|{
                            route.get( |_|  async move {Ok("hello")})
                        })
                    })
            })
    );
}