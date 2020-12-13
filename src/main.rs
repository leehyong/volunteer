#[macro_use]
extern crate rbatis_macro_driver;
use async_std;

use api::api_route;
use import::*;
use setting::{init_mysql_db, CONFIG};
use state::AppState;
use crate::jwt::JwtClaims;

mod api;
mod import;
mod jwt;
mod setting;
mod state;
mod model;
mod util;
mod beijing;

#[async_std::main]
async fn main() -> tide::Result<()> {
    TideLog::with_level(TideLog::LevelFilter::Debug);
    init_mysql_db().await;
    info!("token: {}", JwtClaims::new(1).gen_token());
    let mut app = tide::with_state(AppState::default());
    api_route(&mut app);
    app.listen(&*CONFIG.server).await?;
    Ok(())
}
