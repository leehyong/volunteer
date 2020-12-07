use crate::import::*;
use validator::Validate;
use lazy_static::lazy_static;

lazy_static! {
    static ref ACTIVITY_TYPES: Regex = Regex::new(r"(^gym$)|^(meeting)$|^(concert)$|^(live)$|^(game)$|^(dance)$").unwrap();
    static ref DATETIME_FMT_REG: Regex = Regex::new(r"\d{4}-?\d{2}-?\d{2}\s?(\d{2}:?\d{2}:\d{2})?").unwrap();
}


#[derive(Debug, Validate, Deserialize)]
pub struct ActivityType {
    #[validate(regex(path = "ACTIVITY_TYPES"))]
    pub at: String
}

#[derive(Debug, Validate, Deserialize)]
pub struct Subject {
    #[validate(length(min = 1, max = 512))]
    pub sub: String
}

#[derive(Debug, Validate, Deserialize)]
pub struct ActivityReq {
    // #[validate(regex(path = "DATETIME_FMT_REG"))]
    pub end_time: NaiveDateTime,

    // #[validate(regex(path = "DATETIME_FMT_REG"))]
    pub start_time: Option<NaiveDateTime>,

    #[validate]
    pub activity_types: Option<Vec<ActivityType>>,

    #[validate]
    pub subjects: Option<Vec<Subject>>,
}