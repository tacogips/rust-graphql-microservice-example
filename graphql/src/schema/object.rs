use super::client;
use async_graphql::*;
use chrono::NaiveDateTime;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ArticleStatus {
    Published,
    Draft,
}

pub struct Article {
    pub id: String,
    pub status: ArticleStatus,
    pub text: String,
    pub author_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl Article {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn author(&self) -> Option<User> {
        client::get_user(&self.author_id)
            .await
            .map_err(|e| log::error!("author error {}", e))
            .ok()
    }

    async fn status(&self) -> ArticleStatus {
        self.status
    }

    async fn overview(&self) -> String {
        self.text.clone()
    }

    async fn text(&self) -> Option<String> {
        client::get_article(&self.id).await.map(|a| a.text).ok()
    }

    async fn comments(&self) -> Vec<Comment> {
        client::find_comments(&self.id).await.unwrap()
    }
}

pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

#[Object]
impl User {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn email(&self) -> String {
        self.email.clone()
    }
}

pub struct Comment {
    pub id: String,
    pub text: String,
    pub author_id: String,
    pub created_at: NaiveDateTime,
}

#[Object]
impl Comment {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn text(&self) -> String {
        self.text.clone()
    }

    async fn author(&self) -> Option<User> {
        client::get_user(&self.author_id)
            .await
            .map_err(|e| log::error!("author error {}", e))
            .ok()
    }
}
