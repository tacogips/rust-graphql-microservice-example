use async_graphql::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ArticleStatus {
    Published,
    Draft,
}

pub struct Article {
    id: String,
    status: ArticleStatus,
    overview: String,
    author_id: String,
}

#[Object]
impl Article {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn author(&self) -> User {
        //TODO(tacogips) this is the mock
        User {
            id: self.author_id.to_string(),
            name: format!("dummy_user_{}", self.author_id),
        }
    }

    async fn status(&self) -> ArticleStatus {
        self.status
    }

    async fn overview(&self) -> String {
        self.overview.clone()
    }

    async fn text(&self) -> String {
        //TODO(tacogips) this is the mock
        "dummy text ".to_string()
    }

    async fn first_comment(&self, limit: usize) -> String {
        //TODO(tacogips) this is the mock
        "dummy text ".to_string()
    }
}

pub struct User {
    id: String,
    name: String,
}

#[Object]
impl User {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Comment {
    id: String,
    text: String,
    author_id: String,
}

#[Object]
impl Comment {
    async fn id(&self) -> String {
        self.id.clone()
    }

    async fn text(&self) -> String {
        self.text.clone()
    }

    async fn author(&self) -> User {
        //TODO(tacogips) use Dataloder here
        // https://async-graphql.github.io/async-graphql/en/dataloader.html

        //TODO(tacogips) this is the mock
        User {
            id: self.author_id.to_string(),
            name: format!("dummy_user_{}", self.author_id),
        }
    }
}
