use crate::config::CONFIG;
use jsonwebtoken::DecodingKey;
use once_cell::sync::Lazy;

pub struct Keys {
    pub decoding: DecodingKey,
}

impl Keys {
    fn new() -> Self {
        Self {
            decoding: DecodingKey::from_rsa_components(
               &CONFIG.jwt_key_modulus.clone(),
               &CONFIG.jwt_key_exponent.clone(),
            ).unwrap(),
        }
    }
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    Keys::new()
});
