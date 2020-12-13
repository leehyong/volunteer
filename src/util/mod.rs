mod response;
mod request;
mod util;

pub use response::ResponseUtil;

pub use request::*;
pub use util::*;
pub use util::datetime_util;

use crate::import::Deserialize;
use crate::import::Validate;

#[derive(Copy, Clone, Debug, Deserialize, Default, Validate)]
pub struct  PageReq{
    #[validate(range(min = 1))]
    pub current :u64,

    #[validate(range(min = 1))]
    pub size :u64,
}