use async_graphql::{OutputType, SimpleObject};
use serde::Serialize;

#[derive(Serialize, SimpleObject, Debug)]
pub struct PaginatedResult<T: Send + Sync + OutputType> {
    pub data: Vec<T>,
    pub num_items: i64,
    pub num_pages: i64,
    pub page: i64,
    pub page_items: i64,
}

macro_rules! paginate {
    ($graphql_type:ident, $resource:expr, $where_condition:expr, $page:expr, $limit:expr) => {
        async move {
            let page = $page.unwrap_or(1);
            let limit = $limit.unwrap_or(20);

            let count = $resource.count($where_condition.clone());

            let query = $resource.find_many($where_condition);

            let num_items = count
                .exec()
                .await
                .map_err(|err| AppError::DbError(err.to_string()).into_graphql_error())?;

            let data = query
                .skip(page * limit)
                .take(limit)
                .exec()
                .await
                .map_err(|err| AppError::DbError(err.to_string()).into_graphql_error())?;

            let paginated_result = PaginatedResult::<$graphql_type> {
                data: data.clone().into_iter().map(|item| item.into()).collect(),
                num_items,
                num_pages: (num_items as f32 / limit as f32).ceil() as i64,
                page,
                page_items: data.len() as i64,
            };

            Ok(paginated_result)
        }
    };
}

pub(crate) use paginate;
