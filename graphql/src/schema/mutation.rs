use super::object::*;
use async_graphql::*;

#[derive(InputObject)]
pub struct InputArticle {
    id: String,
    status: ArticleStatus,
    overview: String,
    author_id: String,
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn put_article(&self, article: InputArticle) -> Article {
        unimplemented!()
    }
}
