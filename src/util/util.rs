pub mod datetime_util {
    use crate::import::{BeijingTimezone, DATE_FMT, Datelike,
                        DATETIME_FMT, NaiveDate, NaiveDateTime, Serializer,
                        SysDate, SysDatetime,
                        Utc, UTCDate, UTCDatetime,
    };

    // 其它时区的时间，需要转为tc，再转为北京时间
    pub fn utc2beijing(ut: UTCDatetime) -> SysDatetime {
        SysDatetime::from_utc(ut.naive_local(), BeijingTimezone)
    }

    pub fn naive2beijing(nt: NaiveDateTime) -> SysDatetime {
        SysDatetime::from_utc(nt, BeijingTimezone)
    }

    pub fn datetime2str(dt: &SysDatetime, fmt: Option<&str>) -> String {
        dt.format(fmt.unwrap_or(DATETIME_FMT)).to_string()
    }

    pub fn now2str() -> String {
        // "%Y-%m-%d %H:%M:%S"
        datetime2str(&BeijingTimezone::now(), None)
    }

    pub fn date2str(dt: &SysDate) -> String {
        // "%Y-%m-%d"
        datetime2str(&date2datetime(dt), Some(DATE_FMT))
    }

    pub fn date2datetime(dt: &SysDate) -> SysDatetime {
        SysDatetime::from_utc(dt.naive_local().and_hms(0, 0, 0), BeijingTimezone)
    }

    pub fn datetime2date(dt: &SysDatetime) -> SysDate {
        let nt = dt.naive_local();
        SysDate::from_utc(NaiveDate::from_ymd(nt.year(), nt.month(), nt.day()), BeijingTimezone)
    }

    pub fn now2datestr() -> String {
        datetime2str(&utc2beijing(Utc::now()), Some(DATE_FMT))
    }

    pub fn serialize_datetime<S>(dt: &NaiveDateTime, serializer: S)
                                 -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer {
        serializer.serialize_str(datetime2str(&naive2beijing(dt.clone()), None).as_str())
    }
}

