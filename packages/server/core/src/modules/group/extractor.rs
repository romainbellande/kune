use crate::errors::AppError;
use async_graphql::{Context, Result as GraphQLResult};
use http::HeaderMap;
pub struct GroupExtractor(String);

impl GroupExtractor {
    pub fn from_headers(headers: &HeaderMap) -> Option<Self> {
        match Self::from_headers_with_result(headers) {
            Ok(group) => Some(group),
            Err(_) => None,
        }
    }

    fn from_headers_with_result(headers: &HeaderMap) -> Result<Self, String> {
        let header_value = headers.get("GroupID").ok_or("no group id found")?;
        let header_value_str = header_value.to_str().map_err(|err| err.to_string())?;
        Ok(Self(header_value_str.to_string()))
    }
}

impl ToString for GroupExtractor {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub fn extract_gid(ctx: &Context<'_>) -> GraphQLResult<String> {
    let gid_extractor = ctx
        .data_opt::<GroupExtractor>()
        .ok_or(AppError::GroupIDMissing.into_graphql_error())?;

    Ok(gid_extractor.to_string())
}
