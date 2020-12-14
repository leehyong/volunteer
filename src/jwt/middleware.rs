use anyhow;
use base64;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tide::Middleware;
use async_std::sync::Arc;

use crate::import::*;
use crate::util::datetime_util;
use crate::setting::CONFIG;
use crate::state::AppState;
use crate::model::User;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct JwtClaims {
    user_id: u32,
    exp: i64,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64,
    // Optional. Issued at (as UTC timestamp)
    nbf: i64,
    // Optional. Not Before (as UTC timestamp)
    iss: String,
    // Optional. Issuer
    sub: String, // Optional. Subject (whom token refers to)
}
lazy_static! {
    static ref VALIDATION: Validation = {
        Validation {
            algorithms: vec![Algorithm::HS512],
            iss: Some("leeSystem".to_string()),
            sub: Some("universal".to_string()),
            ..Default::default()
        }
    };
}

impl JwtClaims {
    pub fn new(user_id: u32) -> Self {
        Self::new1(user_id, CONFIG.jwt_expiration as i64)
    }

    pub fn new1(user_id: u32, days: i64) -> Self {
        let now_ts = datetime_util::current_timestamp();
        let valid_time = days * 86400 as i64;
        Self {
            user_id,
            exp: now_ts + valid_time,
            iat: now_ts,
            nbf: now_ts + valid_time,
            iss: "leeSystem".to_owned(),
            sub: "universal".to_owned(),
        }
    }

    pub fn gen_token(&self) -> String {
        self.gen_token1(Self::secret_key().as_bytes())
    }

    pub fn gen_token1(&self, secret: &[u8]) -> String {
        base64::encode(
            encode(
                &Header::new(Algorithm::HS512),
                self,
                &EncodingKey::from_secret(secret),
            )
                .unwrap(),
        )
    }

    fn secret_key() -> &'static str {
        &CONFIG.jwt_key
    }

    pub fn retrive_self1<T: AsRef<str>>(token: T, secret: &[u8]) -> anyhow::Result<Self> {
        let d = base64::decode(token.as_ref())?;
        let t = String::from_utf8(d)?;
        let d = decode::<Self>(&t.trim(), &DecodingKey::from_secret(secret), &*VALIDATION)?;
        Ok(d.claims)
    }

    pub fn retrive_self<T: AsRef<str>>(token: T) -> anyhow::Result<Self> {
        Self::retrive_self1(token, Self::secret_key().as_bytes())
    }
}

// user_id对应的User在AppState的Users里的的过期时间, 30秒
const USER_TIMEOUT: i64 = 30;

pub fn jwt_auth_middleware<'a, >(
    mut ctx: Request<AppState>,
    next: Next<'a, AppState>,
) -> Pin<Box<dyn Future<Output=TideResult> + Send + 'a>>
{
    Box::pin(async move {
        let r = ctx
            .header("token")
            .map(|val| val.as_str())
            .map(|token| JwtClaims::retrive_self(token))
            .map_or_else(|| {
                error!("take token:{}", ctx.header("token").map(|v| v.as_str()).unwrap_or(""), );
                None
            }, |c| c.map_or_else(|e| {
                error!("parse token:{} {}",
                       ctx.header("token").map(|v| v.as_str()).unwrap_or(""),
                       e.to_string());
                None
            }, |c| Some(c)));
        if let Some(c) = r {
            let now = datetime_util::current_timestamp();
            let state = ctx.state().clone();
            let user_id = c.user_id;
            if let Some((time, user)) = state.user(user_id).await {
                let mut tmp_user = user;
                if time + USER_TIMEOUT < now {
                    // 超过缓存有效期，则重写
                    match User::info(user_id).await {
                        Ok(u) => {
                            tmp_user = Arc::new(u);
                            state.set_user((now, tmp_user.clone())).await
                        }
                        Err(e) => {
                            error!("info user:{} {}", user_id, e.to_string());
                            return Ok(http::Response::new(http::StatusCode::Unauthorized).into());
                        }
                    }
                }
                // 把用户存到 每个request 对象的本地状态里
                // set_ext： 对象已经存在时，会返回已存的值；否则返回None
                ctx.set_ext(tmp_user);
                return Ok( next.run(ctx).await);
            }
        }
        // 其他情况一律视为未登陆状态
        Ok(http::Response::new(http::StatusCode::Unauthorized).into())
    })
}

#[test]
fn test_gen_retrive() {
    let claim = JwtClaims::new1(123, 7);
    let secret_key = "leeeasd".as_bytes();
    let token = claim.gen_token1(secret_key);
    dbg!(&token);
    let rd = JwtClaims::retrive_self1(token, secret_key);
    assert_eq!(claim, rd.unwrap())
}

#[test]
#[should_panic]
fn test_gen_retrive1() {
    let claim = JwtClaims::new1(123, 7);
    let secret_key = "leeeasd".as_bytes();
    let token = claim.gen_token1(secret_key);
    let rd = JwtClaims::retrive_self1(token + "das", secret_key);
    assert_ne!(claim, rd.unwrap())
}
