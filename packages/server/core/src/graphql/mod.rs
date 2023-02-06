mod schema;
use crate::config::CONFIG;
use async_graphql::{http::GraphiQLSource, Guard};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::header::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use schema::{get_schema, AppSchema};
use sea_orm::DatabaseConnection;
use crate::modules::auth::{token::AccessTokenRaw, guard::AuthGuard};

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(access_token_raw) = AccessTokenRaw::from_header(&headers) {
        req = req.data(access_token_raw);
    }

    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    let endpoint = format!("http://0.0.0.0:{}/graphql", CONFIG.port);

    Html(GraphiQLSource::build().endpoint(&endpoint).finish())
}

pub fn router(conn: DatabaseConnection) -> Router {
    let schema = get_schema(conn);

    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
}
