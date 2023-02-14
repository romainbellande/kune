mod web_error;
use thiserror::Error;
use async_graphql::{Error as GraphQLError, ErrorExtensions};
use self::web_error::WebError;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum AppError {
  #[error("user {0} not found")]
  UserNotFound(String),

  #[error("error during user creation: {0}")]
  UserCreationError(String),

  // jwt token

  #[error("missing access token")]
  MissingAccessToken,

  #[error("invalid token: {0}")]
  InvalidToken(String),

  #[error("missing jwk in jwks")]
  MissingJwk,

  #[error("token doesn't have a `kid` header field")]
  JwtNoKid,

  #[error("jwk must have a RSA algorithm")]
  JwkInvalidRsaAlgorithm,
}

impl AppError {
  pub fn get_code(&self) -> String {
    let str = match self {
      Self::UserNotFound(_) => "USER_NOT_FOUND",
      Self::UserCreationError(_) => "USER_CREATION_ERROR",
      Self::MissingAccessToken => "MISSING_ACCESS_TOKEN",
      Self::InvalidToken(_) => "INVALID_TOKEN",
      Self::MissingJwk => "MISSING_JWK",
      Self::JwtNoKid => "JWT_NO_KID",
      Self::JwkInvalidRsaAlgorithm => "JWK_INVALID_RSA_ALGORITHM",
    };

    str.to_string()
  }

  pub fn into_graphql_error(self) -> GraphQLError {
    let web_error: WebError = self.into();
    web_error.extend()
  }
}

impl Into<WebError> for AppError {
  fn into(self) -> WebError {
      WebError {
        code: self.get_code(),
        message: self.to_string()
      }
  }
}
