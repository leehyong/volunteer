mod activity;

use tide::Server;
use crate::import::*;
use crate::state::AppState;
use crate::jwt::jwt_auth_middleware;
use crate::api::activity::ActivityApi;

pub fn api_route(app: &mut Server<AppState>){
    app.register(
        root()
            .at("api/v1/", |route| {
                let route = route
                    .at("activity", |route| {
                        route
                            .get(ActivityApi::list)
                            .post(ActivityApi::new)
                            .at("/{}", |route| {
                                route.get(ActivityApi::detail)
                            })
                    });

                    // route.with(jwt_auth_middleware, |route|{
                    //     route.at("/apply2", |route|{
                    //         route.get( |_|  async move {Ok("hello")})
                    //     })
                    // })
                route
            })
    );
    app.with(jwt_auth_middleware);
}