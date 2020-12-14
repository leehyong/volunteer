use crate::import::*;
use validator::Validate;


#[derive(Debug, Validate, Deserialize)]
pub struct UserReq {
    #[validate(range(min = 1))]
    pub user_id: u32,
}