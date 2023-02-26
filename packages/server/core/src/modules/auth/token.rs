use super::keys::get_jwk;
use crate::errors::AppError;
use crate::CONFIG;
use async_graphql::Result;
use axum::headers::{authorization::Bearer, Authorization, HeaderMapExt};
use http::HeaderMap;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters::RSA, Jwk, RSAKeyParameters},
    DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessToken {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub aud: Vec<String>,
    pub iss: String,
    pub azp: String,
    pub scope: String,
}

impl AccessToken {
    pub async fn from_string(value: String) -> Result<Self> {
        let kid = Self::get_kid(value.clone())?;

        let jwk = get_jwk(kid).await?;

        let rsa = Self::extract_rsa(&jwk)?;

        let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();

        decode::<Self>(&value, &decoding_key, &Self::validation(&jwk))
            .map(|value| value.claims)
            .map_err(|err| AppError::InvalidToken(err.to_string()).into_graphql_error())
    }

    fn validation(jwk: &Jwk) -> Validation {
        let mut validation = Validation::new(jwk.common.algorithm.unwrap());

        validation.leeway = 5;

        validation.set_audience(&[CONFIG.auth0_audience.clone()]);

        let issuer = format!("https://{}/", CONFIG.auth0_domain.clone());

        validation.set_issuer(&[issuer]);

        validation
    }

    fn get_kid(value: String) -> Result<String> {
        let header = decode_header(&value)?;

        match header.kid {
            Some(kid) => Ok(kid),
            None => Err(AppError::JwtNoKid.into_graphql_error()),
        }
    }

    fn extract_rsa(jwk: &Jwk) -> Result<&RSAKeyParameters> {
        match &jwk.algorithm {
            RSA(rsa) => Ok(rsa),
            _ => Err(AppError::JwkInvalidRsaAlgorithm.into_graphql_error()),
        }
    }
}

#[derive(Clone)]
pub struct AccessTokenRaw(pub String);

impl AccessTokenRaw {
    pub fn from_headers(headers: &HeaderMap) -> Option<Self> {
        headers
            .typed_get::<Authorization<Bearer>>()
            .map(|value| Self(value.0.token().to_string()))
    }
}

impl ToString for AccessTokenRaw {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
