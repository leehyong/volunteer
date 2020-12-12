use crate::import::*;
use validator::Validate;
use lazy_static::lazy_static;

lazy_static! {
    static ref ACTIVITY_TYPES: Regex = Regex::new(r"(^gym$)|^(meeting)$|^(concert)$|^(live)$|^(game)$|^(dance)$|^(kongfu)$").unwrap();
    static ref DATETIME_FMT_REG: Regex = Regex::new(r"\d{4}-?\d{2}-?\d{2}\s?(\d{2}:?\d{2}:\d{2})?").unwrap();
}

#[derive(Debug, Validate, Deserialize)]
pub struct ActivityReq {
    // #[validate(regex(path = "DATETIME_FMT_REG"))]
    pub end_date: NaiveDate,

    // #[validate(regex(path = "DATETIME_FMT_REG"))]
    pub start_date: Option<NaiveDate>,

    #[validate(regex(path = "ACTIVITY_TYPES"))]
    pub activity_type: Option<String>,

    #[validate(length(min = 1, max = 512))]
    pub subject: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct NewActivityReq {
    // #[validate(regex(path = "DATETIME_FMT_REG"))]
    pub end_time: NaiveDate,

    // #[validate(regex(path = "DATETIME_FMT_REG"))]
    pub start_time: Option<NaiveDate>,

    #[validate(regex(path = "ACTIVITY_TYPES"))]
    pub activity_type: Option<String>,

    #[validate(length(min = 1, max = 512))]
    pub subject: Option<String>,
}