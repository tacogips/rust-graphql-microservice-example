use super::object::*;
use async_graphql::*;

use super::client;

#[derive(InputObject)]
pub struct InputArticle {
    id: String,
    status: ArticleStatus,
    overview: String,
    author_id: String,
}

pub struct Mutation;

struct DeleteOperationResult {
    pub id: String,
}

#[Object]
impl DeleteOperationResult {
    pub async fn id(&self) -> String {
        self.id.clone()
    }
}

#[Object]
impl Mutation {
    async fn post_article(&self, article: InputArticle) -> String {
        "dummy".to_string()
    }

    async fn delete_article(&self, article_id: String) -> DeleteOperationResult {
        client::delete_article(&article_id)
            .await
            .unwrap_or_else(|e| {
                log::error!("delete article error:{}", e);
            });
        DeleteOperationResult { id: article_id }
    }
}
