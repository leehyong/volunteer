mod activity;
mod user;

use crate::import::*;
pub use activity::{UpdateActivityReq, NewActivityReq, ActivityReq};
pub use user::UserReq;


#[derive(Copy, Clone, Debug, Deserialize, Default, Validate)]
pub struct  PageReq{
    #[validate(range(min = 1))]
    pub current :u64,

    #[validate(range(min = 1))]
    pub size :u64,
}