use super::object::*;
use async_graphql::*;
#[derive(InputObject)]
struct Pagination {
    skip: usize,
    limit: usize,
}

fn default_article_pagination() -> Pagination {
    Pagination { skip: 0, limit: 10 }
}

pub struct Query;

#[Object]
impl Query {
    async fn find_articles(
        &self,
        #[graphql(default_with = "default_article_pagination()")] pagination: Pagination,
    ) -> Vec<Article> {
        //TODO(tacogips) this is just mock
        vec![]
    }
}
