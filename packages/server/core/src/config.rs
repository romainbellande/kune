use crate::graphql::types::user::CreateUserDto;
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
    pub auth0_domain: String,
    pub super_admin: CreateUserDto,
}

impl Config {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            rust_env: RustEnv::new(env::var("RUST_ENV").expect("RUST_ENV must be set")),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse::<u16>()
                .expect("PORT is not valid"),
            database_url,
            auth0_domain: env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set"),
            super_admin: CreateUserDto {
                name: env::var("SUPER_ADMIN_NAME").expect("SUPER_ADMIN_NAME must be set"),
                external_id: env::var("SUPER_ADMIN_EXTERNAL_ID").expect("SUPER_ADMIN_EXTERNAL_ID must be set"),
            },
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
