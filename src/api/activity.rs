use crate::import::*;
use crate::AppState;

pub struct ActivityApi;


impl ActivityApi {
    pub async fn list(mut req: Request<AppState>) -> TideResult
    {
        Ok("list".into())
    }

    pub async fn detail(mut req: Request<AppState>) -> TideResult
    {
        Ok("detail".into())
    }

    pub async fn new(mut req: Request<AppState>) -> TideResult
    {
        Ok("new".into())
    }
}