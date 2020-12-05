pub use async_trait::async_trait;
pub use rbatis;
pub use std::future::Future;
pub use std::pin::Pin;
pub use tide::http;
pub use tide::log::{debug, error, info, trace, warn};
pub use tide::prelude::*;
pub use tide::prelude::*;
pub use tide::utils::After;
pub use tide::{log as TideLog, Middleware, Next, Request, Response};

pub type TideResult = tide::Result;

pub const API_PATH: &'static str = "/api/v1";
pub const ADMIN_PATH: &'static str = "/admin/v1";
