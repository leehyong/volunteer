use std::env;

use fern;
use envy;
use lazy_static::lazy_static;
use serde::Deserialize;

use dotenv;

use crate::import::*;

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    static ref ENV_CONFIG_FILE:String = get_env_config_file();
    pub static ref CONFIG: Config = get_config();
    // pub static ref CACHE: Cache = init_redis_cache();
    pub static ref DB: rbatis::rbatis::Rbatis = rbatis::rbatis::Rbatis::new();
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub env: String,
    pub testing: bool,

    // auth
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


pub fn setup_logger() -> Result<(), fern::InitError> {
    //! With fern, we can:
// Configure logger at runtime
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply()?;

    // and log using log crate macros!
    Ok(())
}

pub async fn init_mysql_db() {
    let mut opt = rbatis::core::db::PoolOptions::new();
    opt.max_connections = CONFIG.max_connections;
    opt.min_connections = CONFIG.min_connections;
    DB.link_opt(&*CONFIG.database_url, &opt).await.unwrap();
}


// todo  set up redis

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = get_config();
        assert_ne!(config.server, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.server, "".to_string());
    }
}