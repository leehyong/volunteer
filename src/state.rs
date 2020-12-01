use async_std::sync::Arc;
use tide::http::headers::ToHeaderValues;

#[derive(Clone)]
pub struct AppState(Arc<InnerAppState>);

impl Default for AppState{
    fn default() -> Self{
        AppState(Arc::new(InnerAppState::new()))
    }
}

struct InnerAppState{
    name:String,
}

impl InnerAppState {
    fn new() -> Self{
        InnerAppState{
            name:"leehuayong".to_string()
        }
    }
}