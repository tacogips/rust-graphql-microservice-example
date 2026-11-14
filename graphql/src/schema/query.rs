use super::client;
use super::object::*;
use async_graphql::*;
use log;

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
        client::find_articles_with_overview()
            .await
            .unwrap_or_else(|e| {
                log::error!("fetch article error:{}", e);
                vec![]
            })
    }
}
