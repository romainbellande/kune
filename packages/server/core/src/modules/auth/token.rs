use axum::{headers::{Authorization, authorization::{Bearer, Credentials}, HeaderMapExt}, extract::{TypedHeader, FromRequestParts}};
use http::HeaderMap;
use async_graphql::Error;
use jsonwebtoken::{decode, Validation};
use serde::{Serialize, Deserialize};
use super::keys::KEYS;
use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessToken {
  pub sub: String,
  pub exp: String,
  pub aud: String,
  pub iss: String,
}

impl AccessToken {
  pub fn from_string(value: String) -> Result<Self, Error> {
    decode::<Self>(&value, &KEYS.decoding, &Validation::default())
      .map(|value| value.claims)
      .map_err(|_| AppError::InvalidToken.into_graphql_error())
  }
}

#[derive(Clone)]
pub struct AccessTokenRaw(pub String);

impl AccessTokenRaw {
  pub fn from_header(headers: &HeaderMap) -> Option<Self> {
    headers.typed_get::<Authorization<Bearer>>()
      .map(|value| Self(value.0.token().to_string()))
  }
}

impl ToString for AccessTokenRaw {
  fn to_string(&self) -> String {
      self.0.clone()
  }
}
