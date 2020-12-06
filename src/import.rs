pub use async_trait::async_trait;
pub use rbatis::{rbatis::{Rbatis, RbatisOption},
                 core::Error as DbError,
                 core::Result as DbResult,
                 core::db::DBPoolOptions};
pub use std::future::Future;
pub use std::pin::Pin;
pub use tide::http;
pub use tide::log::{debug, error, info, trace, warn};
pub use tide::prelude::*;

pub use tide::utils::After;
pub use tide::{log as TideLog,
               StatusCode, Middleware,
               http::Mime,
               Next, Request, Response};
pub use chrono::prelude::*;
pub use tide::prelude::*;
pub use crate::setting::DB;
pub type TideResult = tide::Result;

pub const API_PATH: &'static str = "/api/v1";
pub const ADMIN_PATH: &'static str = "/admin/v1";

// 注意前面的 空格符不能去掉， 不然会报sql语法错误
pub const LIMIT_NUM_SQL: &'static str = " limit 50";