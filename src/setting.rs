use std::env;

use dotenv;
#[cfg(test)]
use envmnt;
use envy;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::import::*;

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    static ref ENV_CONFIG_FILE:String = get_env_config_file();
    pub static ref CONFIG: Config = get_config();
    // pub static ref CACHE: Cache = init_redis_cache();
    pub static ref DB: Rbatis = Rbatis::new();
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub env: String,
    pub testing: bool,

    // auth
    // expiration days
    pub jwt_expiration: i64,
    pub jwt_key: String,
    // db
    pub database_url: String,
    pub min_connections: u32,
    pub max_connections: u32,

    // redis
    pub redis_url: String,

    //server
    pub server: String,

    // session
    pub session_key: String,
    pub session_name: String,
    pub session_secure: bool,
    pub session_timeout: i64,
}

fn get_env_config_file() -> String {
    if (cfg!(test)) {
        let r = envmnt::get_or("CONF", "config/local.env").to_string();
        dbg!(&r);
        return r;
    }
    let mut args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("error argument! usage {} [env]", &args[0])
    }
    args.remove(1)
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    dotenv::from_filename(&*ENV_CONFIG_FILE).unwrap();
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:?}", error),
    }
}

pub async fn init_mysql_db() {
    let mut opt = DBPoolOptions::new();
    opt.max_connections = CONFIG.max_connections;
    opt.min_connections = CONFIG.min_connections;
    DB.link_opt(&*CONFIG.database_url, &opt).await.unwrap();
}

// todo  set up redis

#[async_std::test]
async fn test_config() {
    dbg!(&CONFIG.server);
    dbg!(&CONFIG.env);
    assert!(CONFIG.server.len() > 0);
    assert!(CONFIG.env.len() > 0);
    assert!(CONFIG.jwt_key.len() > 0);
}

#[async_std::test]
async fn test_mysql_db() {
    init_mysql_db().await;
    let py_sql = r#"show databases;"#;
    DB.
        py_exec(
            "",
            py_sql,
            &"".to_owned(),
        ).await.unwrap();
    let py_sql = r#"show tables;"#;
    DB.
        py_exec(
            "",
            py_sql,
            &"".to_owned(),
        ).await.unwrap();
}