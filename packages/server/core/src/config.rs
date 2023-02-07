use once_cell::sync::Lazy;
use std::env;

#[derive(Clone, Eq, PartialEq)]
pub enum RustEnv {
    Development,
    Production,
}

impl RustEnv {
    pub fn new(value: String) -> Self {
        match value.as_str() {
            "development" => Self::Development,
            "production" => Self::Production,
            value => panic!("RUST_ENV {value} invalid"),
        }
    }
}

pub struct Config {
    pub rust_env: RustEnv,
    pub port: u16,
    pub database_url: String,
    pub jwt_key_modulus: String,
    pub jwt_key_exponent: String,
    pub auth0_domain: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            rust_env: RustEnv::new(env::var("RUST_ENV").expect("RUST_ENV must be set")),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse::<u16>()
                .expect("PORT is not valid"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_key_modulus: env::var("JWT_KEY_MODULUS").expect("JWT_KEY_MODULUS must be set"),
            jwt_key_exponent: env::var("JWT_KEY_EXPONENT").expect("JWT_KEY_EXPONENT must be set"),
            auth0_domain: env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set"),
        }
    }

    pub fn is_dev(&self) -> bool {
        self.rust_env == RustEnv::Development
    }

    pub fn is_production(&self) -> bool {
        self.rust_env == RustEnv::Production
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);
