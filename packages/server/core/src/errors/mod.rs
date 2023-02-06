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

  #[error("missing access token")]
  MissingAccessToken,

  #[error("invalid token")]
  InvalidToken,
}

impl AppError {
  pub fn get_code(&self) -> String {
    let str = match self {
      Self::UserNotFound(_) => "USER_NOT_FOUND",
      Self::UserCreationError(_) => "USER_CREATION_ERROR",
      Self::MissingAccessToken => "MISSING_ACCESS_TOKEN",
      Self::InvalidToken => "INVALID_TOKEN"
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
