use async_graphql::{Error as GraphQLError, ErrorExtensions};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct WebError {
    pub code: String,

    pub message: String,
}

impl Display for WebError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.message)
    }
}

impl ErrorExtensions for WebError {
    fn extend(&self) -> GraphQLError {
        GraphQLError::new(self.message.clone()).extend_with(|_, e| {
            e.set("code", self.code.clone());
        })
    }
}
