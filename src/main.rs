use async_std;

use api::api_route;
use import::*;
use setting::{setup_logger, init_mysql_db, CONFIG};
use state::AppState;
use jwt::JwtClaims;

mod jwt;
mod api;
mod import;
mod state;
mod setting;
use base64::encode;

#[async_std::main]
async fn main() -> tide::Result<()> {
    setup_logger().unwrap();
    init_mysql_db().await;
    info!("token: {}", JwtClaims::new(123).gen_token());
    let mut app = tide::with_state(AppState::default());
    api_route(&mut app);
    app.listen(&*CONFIG.server).await?;
    Ok(())
}
