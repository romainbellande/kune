mod schema;
pub mod types;

use crate::modules::group::GroupExtractor;
use crate::{modules::auth::token::AccessTokenRaw, State};
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::header::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use schema::{get_schema, AppSchema};

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(access_token_raw) = AccessTokenRaw::from_headers(&headers) {
        req = req.data(access_token_raw);
    }

    if let Some(group_id_extractor) = GroupExtractor::from_headers(&headers) {
        req = req.data(group_id_extractor);
    }

    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub fn router(state: State) -> Router {
    let schema = get_schema(state);

    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
}
