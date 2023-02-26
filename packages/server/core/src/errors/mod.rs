mod web_error;
use self::web_error::WebError;
use async_graphql::{Error as GraphQLError, ErrorExtensions};
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum AppError {
    // user
    #[error("missing user in context")]
    MissingUserInContext,

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
    // groups
    #[error("owner not found")]
    OwnerNotFound,

    #[error("error during group creation: {0}")]
    GroupCreationError(String),

    #[error("error during user groups fetch: {0}")]
    GroupsFindError(String),

    #[error("error during groups counting: {0}")]
    GroupsCountError(String),

    #[error("missing group id")]
    GroupIDMissing,

    // ACL
    #[error("user {user:?} is not allowed to {permission:?} for resource {resource:?} in group {group:?}")]
    AclError {
        user: String,
        resource: String,
        group: String,
        permission: String,
    },

    #[error("an error occured while creating a new policy: {0}")]
    AclCreatePolicyError(String),

    #[error("an error occured during acl enforcer attempt: {0}")]
    AclEnforceError(String),
}

impl AppError {
    pub fn get_code(&self) -> String {
        let str = match self {
            Self::MissingUserInContext => "MISSING_USER_IN_CONTEXT",
            Self::UserNotFound(_) => "USER_NOT_FOUND",
            Self::UserCreationError(_) => "USER_CREATION_ERROR",
            Self::MissingAccessToken => "MISSING_ACCESS_TOKEN",
            Self::InvalidToken(_) => "INVALID_TOKEN",
            Self::MissingJwk => "MISSING_JWK",
            Self::JwtNoKid => "JWT_NO_KID",
            Self::JwkInvalidRsaAlgorithm => "JWK_INVALID_RSA_ALGORITHM",
            Self::OwnerNotFound => "OWNER_NOT_FOUND",
            Self::GroupCreationError(_) => "GROUP_CREATION_ERROR",
            Self::GroupsFindError(_) => "GROUPS_FIND_ERROR",
            Self::GroupsCountError(_) => "GROUPS_COUNT_ERROR",
            Self::GroupIDMissing => "GROUP_ID_MISSING",
            Self::AclError { .. } => "ACL_ERROR",
            Self::AclCreatePolicyError(_) => "ACL_CREATE_POLICY_ERROR",
            Self::AclEnforceError(_) => "ACL_ENFORCE_ERROR",
        };

        str.to_string()
    }

    pub fn into_graphql_error(self) -> GraphQLError {
        let web_error: WebError = self.into();
        web_error.extend()
    }
}

impl From<AppError> for WebError {
    fn from(val: AppError) -> Self {
        WebError {
            code: val.get_code(),
            message: val.to_string(),
        }
    }
}
