use anyhow;
use base64;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tide::Middleware;

use crate::import::*;
use crate::setting::CONFIG;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct JwtClaims {
    user_id: u64,
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
    pub fn new(user_id: u64) -> Self {
        Self::new1(user_id, CONFIG.jwt_expiration as i64)
    }

    pub fn new1(user_id: u64, days: i64) -> Self {
        let now_ts = Utc::now().timestamp();
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

pub fn jwt_auth_middleware<'a, State>(
    mut ctx: Request<State>,
    next: Next<'a, State>,
) -> Pin<Box<dyn Future<Output = TideResult> + Send + 'a>>
where
    State: Clone + Send + Sync + 'static,
{
    Box::pin(async move {
        let r = ctx
            .header("token")
            .map(|val| val.as_str())
            .map(|token| JwtClaims::retrive_self(token))
            .map(|claim| match claim {
                Ok(c) => {
                    ctx.set_ext(c);
                    Some(())
                }
                Err(e) => {
                    error!(
                        "token:{} {}",
                        ctx.header("token").map(|v| v.as_str()).unwrap_or(""),
                        e.to_string()
                    );
                    None
                }
            });
        if r.unwrap().is_some() {
            let response = next.run(ctx).await;
            return Ok(response);
        }
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
