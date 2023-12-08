use crate::{Error, Result};
use std::{env, net::SocketAddr, sync::OnceLock};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Config {
    pub DATABASE_URL: String,
    pub ADDRESS: SocketAddr,
    pub TOKEN_HEADER: String,
    pub JWT_SECRET: String,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}
impl Config {
    fn load_from_env() -> Result<Config> {
        let addr_str = get_env("UDESK_ADDRESS")?;
        let addr = addr_str
            .parse()
            .map_err(|ex| Error::ConfigInvalidAddress(addr_str))?;

        Ok(Config {
            TOKEN_HEADER: get_env("UDESK_TOKEN_HEADER")?,
            JWT_SECRET: get_env("UDESK_JWT_SECRET")?,
            DATABASE_URL: get_env("DATABASE_URL")?,
            ADDRESS: addr,
        })
    }
}

fn get_env(var: &'static str) -> Result<String> {
    env::var(var).map_err(|ex| Error::ConfigMissingEnv(var.to_string()))
}
