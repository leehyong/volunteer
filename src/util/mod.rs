mod response;
mod request;
pub mod util;
pub mod datetime_util;

pub use response::ResponseUtil;
pub use request::*;

use crate::import::Deserialize;
use crate::import::Validate;

#[derive(Copy, Clone, Debug, Deserialize, Default, Validate)]
pub struct  PageReq{
    #[validate(range(min = 1))]
    pub current :u64,

    #[validate(range(min = 1))]
    pub size :u64,
}