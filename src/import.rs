pub use async_trait::async_trait;
pub use rbatis::{rbatis::{Rbatis, RbatisOption},
                 core::Error as DbError,
                 core::Result as DbResult,
                 core::db::DBPoolOptions};
pub use std::future::Future;
pub use std::pin::Pin;
pub use tide::http;
pub use chrono::prelude::*;
pub use chrono::{Datelike, NaiveDate, NaiveDateTime, Offset, LocalResult};
pub use serde::{Serialize, Deserialize, Serializer, Deserializer};
pub use serde_json::{json, Value, from_str};
pub use tide::log::{debug, error, info, trace, warn};
pub use tide::{log as TideLog,
               convert,
               StatusCode, Middleware, http::Mime,
               Next, Request, Response};
pub use regex::Regex;
pub use async_std::task::block_on;
// A trait that the Validate derive will impl
pub use validator::{Validate, ValidationError, ValidationErrors};
pub use crate::setting::DB;

pub type TideResult = tide::Result;

pub const API_PATH: &'static str = "/api/v1";
pub const ADMIN_PATH: &'static str = "/admin/v1";

// 注意前面的 空格符不能去掉， 不然会报sql语法错误
pub const LIMIT_NUM_SQL: &'static str = " limit 50";

// 为了保证系统所有的时间操作不会出现混乱，因此系统里的所有时间均以北京所在时区进行时间存储
// 如果请求所在是时区不是北京时区，本系统会自动转换为北京时区，并进行数据的存储，
// 这样mysql数据库里的 datetime 类型存储的均是北京时区相关的时间数据
// （mysql 里的 datetime 存储的是与时区无关的原始时间。）
pub use crate::beijing::{SysDatetime, SysDate, BeijingTimezone};
pub type UTCDatetime = DateTime<Utc>;
pub type UTCDate = Date<Utc>;
pub const DATETIME_FMT: &'static str = "%Y-%m-%d %H:%M:%S";
pub const DATE_FMT: &'static str = "%Y-%m-%d";
