use std::fmt;

use crate::import::*;

pub type SysDatetime = DateTime<BeijingTimezone>;
pub type SysDate = Date<BeijingTimezone>;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BeijingTimezone;


impl BeijingTimezone {
    /// Returns a `Date` which corresponds to the current date.
    pub fn today() -> SysDate {
        SysDate::from_utc(Utc::today().naive_local(), BeijingTimezone)
    }

    pub fn now() -> SysDatetime {
        SysDatetime::from_utc(Utc::now().naive_local(), BeijingTimezone)
    }
}

// 以下代码抄自 chrono的 UTC
impl TimeZone for BeijingTimezone {
    type Offset = BeijingTimezone;

    fn from_offset(_state: &Self::Offset) -> BeijingTimezone {
        BeijingTimezone
    }

    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<BeijingTimezone> {
        LocalResult::Single(BeijingTimezone)
    }
    fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<BeijingTimezone> {
        LocalResult::Single(BeijingTimezone)
    }

    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> BeijingTimezone {
        BeijingTimezone
    }
    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> BeijingTimezone {
        BeijingTimezone
    }
}

impl Offset for BeijingTimezone {
    fn fix(&self) -> FixedOffset {
        FixedOffset::east(8 * 3600)
    }
}

impl fmt::Debug for BeijingTimezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GMT+8")
    }
}

impl fmt::Display for BeijingTimezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GMT+8")
    }
}