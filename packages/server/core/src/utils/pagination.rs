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

// impl<T: Send + Sync + OutputType> PaginatedResult<T> {
//     pub async fn new<'a>(
//         query: impl PaginatedQuery<'a>,
//         page: i64,
//         limit: i64,
//     ) -> Self {
//       query.set_skip(page * limit);
//       query.set_take(limit);
//       query.graphql()
//       let num_pages = paginator.num_pages().await.ok().unwrap();
//       let num_items = paginator.num_items().await.ok().unwrap();

//         Self {
//             page,
//             num_pages,
//             num_items,
//             page_items: data.len(),
//             data,
//         }
//     }
// }
