mod jwt;
mod api;
mod import;
mod state;
mod setting;

use async_std;
use import::*;
use api::api_route;
use state::AppState;
use setting::setup_logger;

#[async_std::main]
async fn main() -> tide::Result<()> {
    setup_logger().unwrap();
    let mut app = tide::with_state(AppState::default());
    api_route(&mut app);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
