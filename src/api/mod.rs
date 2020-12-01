mod activity;

use tide::Server;
use crate::import::*;
use crate::state::AppState;
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
            })
    );
}