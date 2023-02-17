mod schema;
pub mod types;
use crate::modules::auth::token::AccessTokenRaw;
use crate::{config::CONFIG, prisma::PrismaClient};
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

    if let Some(access_token_raw) = AccessTokenRaw::from_header(&headers) {
        req = req.data(access_token_raw);
    }

    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    let endpoint = format!("http://0.0.0.0:{}/graphql", CONFIG.port);

    Html(GraphiQLSource::build().endpoint(&endpoint).finish())
}

pub fn router(db: PrismaClient) -> Router {
    let schema = get_schema(db);

    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
}
