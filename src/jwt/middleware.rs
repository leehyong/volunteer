use anyhow;
use base64;
use chrono::Utc;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tide::{Error, http, Middleware, Next, Request, Result as TideResult, StatusCode};

// use crate::auth::AuthMiddleware;

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
    sub: String,         // Optional. Subject (whom token refers to)
}

impl JwtClaims {
    pub fn new(user_id: u64) -> Self {
        let now_ts = Utc::now().timestamp();
        let valid_time = 86400 * 300;
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
        base64::encode(encode(
            &Header::new(Algorithm::HS512),
            self,
            &EncodingKey::from_secret(Self::secret_key().as_bytes()),
        ).unwrap())
    }

    fn secret_key() -> &'static str {
        "leehuayong"
    }

    pub fn retrive_self<T: AsRef<str>>(token: T) -> anyhow::Result<Self> {
        base64::decode(token.as_ref()).map(|val| {
            String::from_utf8(val)
        }).map(|val| {
            let validation = Validation {
                algorithms: vec![Algorithm::HS512],
                iss: Some("leeSystem".to_string()),
                sub: Some("universal".to_string()),
                ..Default::default()
            };
            decode::<Self>(&val.unwrap(),
                           &DecodingKey::from_secret(Self::secret_key().as_bytes()),
                           &validation)
        }).map_err(|e| e.into())
            .and_then(|val| Ok(val.unwrap().claims))
    }
}

// impl <State: Clone + Send + Sync + 'static> AuthMiddleware<State> for JwtClaims{
//     fn authorized(&self, ctx: &mut Request<State>) -> bool{
//         let r = ctx.header("token")
//             .map(|val| val.as_str())
//             .map(|token| JwtClaims::retrive_self(token))
//             .map(|claim| {
//                 claim.map(|claim| ctx.set_ext(claim))
//                     .map_err(|_| ())
//                     .map(|_| Some(()));
//             });
//         let mut success = false;
//         if r.is_none() {
//             // 没有成功获取到token对应到claim， 则判断是否是登陆时到请求，或者一些不需要授权的接口
//             let url = ctx.url().path();
//             if url.starts_with("/api/v1/login") {
//                 if ctx.method() == http::Method::Post
//                     || ctx.method() == http::Method::Put {
//                     success = true;
//                 }else{
//                     success = false;
//                 }
//             }
//             // todo 加入一些不需要鉴权的 url
//         }
//         success
//     }
// }


#[test]
fn test_gen_retrive() {
    let claim = JwtClaims::new(123);
    let token = claim.gen_token();
    let rd = JwtClaims::retrive_self(token);
    assert_eq!(claim, rd.unwrap())
}

#[test]
#[should_panic]
fn test_gen_retrive1() {
    let claim = JwtClaims::new(123);
    let token = claim.gen_token();
    let rd = JwtClaims::retrive_self(token + "das");
    assert_ne!(claim, rd.unwrap())
}

