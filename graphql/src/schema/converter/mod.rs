pub mod response {

    pub use models::article::*;
    pub use models::comment::*;
    pub use models::user::*;
}

pub use super::object::{Article, ArticleStatus, Comment, User};

impl Into<ArticleStatus> for response::ArticleStatus {
    fn into(self) -> ArticleStatus {
        match self {
            response::ArticleStatus::Published => ArticleStatus::Published,
            response::ArticleStatus::Draft => ArticleStatus::Draft,
        }
    }
}

impl Into<Article> for response::ArticleRow {
    fn into(self) -> Article {
        Article {
            id: self.id.to_string(),
            status: self.status.into(),
            text: self.text.unwrap_or("".to_string()),
            author_id: self.author_id.to_string(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Article {
    pub fn from(articles: Vec<response::ArticleRow>) -> Vec<Article> {
        articles.into_iter().map(Into::into).collect()
    }
}

impl Into<User> for response::UserRow {
    fn into(self) -> User {
        User {
            id: self.id.to_string(),
            name: self.name,
            email: self.email,
        }
    }
}

impl User {
    pub fn from(users: Vec<response::UserRow>) -> Vec<User> {
        users.into_iter().map(Into::into).collect()
    }
}

impl Into<Comment> for response::CommentRow {
    fn into(self) -> Comment {
        Comment {
            id: self.id.to_string(),
            text: self.text,
            author_id: self.author_id.to_string(),
            created_at: self.created_at,
        }
    }
}

impl Comment {
    pub fn from(comments: Vec<response::CommentRow>) -> Vec<Comment> {
        comments.into_iter().map(Into::into).collect()
    }
}
