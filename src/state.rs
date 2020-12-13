use std::collections::HashMap;

use async_std::sync::Arc;
use async_std::sync::Mutex;
use tide::http::headers::ToHeaderValues;

use crate::model::User;

#[derive(Clone)]
pub struct AppState(Arc<Mutex<InnerAppState>>);

impl Default for AppState {
    fn default() -> Self {
        AppState(Arc::new(Mutex::new(InnerAppState::new())))
    }
}

struct InnerAppState {
    users:HashMap<u32, (i64, Arc<User>)> // (user_id, (用户更新时间， User))
}

impl InnerAppState {
    fn new() -> Self {
        InnerAppState {
            users:HashMap::new()
        }
    }
}

impl AppState{
    pub async fn set_user(&self, user:(i64, Arc<User>)){
        let mut mg =  self.0.lock().await;
        mg.users.insert(user.1.id, user);
    }

    pub async fn user(&self, user_id:u32) -> Option<(i64, Arc<User>)>{
        let mg =  self.0.lock().await;
        mg.users.get(&user_id).cloned()
    }

    pub async fn remove_user(&self, user_id:u32) -> Option<(i64, Arc<User>)>{
        let mut mg =  self.0.lock().await;
        mg.users.remove(&user_id)
    }

}


