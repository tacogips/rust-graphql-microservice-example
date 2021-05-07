pub mod article {

    use sqlx::{
        self,
        types::{chrono::NaiveDateTime, Uuid},
    };

    use serde::{Deserialize, Serialize};

    #[derive(sqlx::Type, Debug, Serialize, Deserialize)]
    #[sqlx(type_name = "article_status", rename_all = "lowercase")]
    pub enum ArticleStatus {
        Published,
        Draft,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ArticleRow {
        pub id: Uuid,
        pub status: ArticleStatus,
        pub text: Option<String>,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,

        pub author_id: Uuid,
    }
}

pub mod user {

    use sqlx::{
        self,
        types::{chrono::NaiveDateTime, Uuid},
    };

    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UserRow {
        pub id: Uuid,
        pub name: String,
        pub email: String,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }
}

pub mod comment {

    use sqlx::{
        self,
        types::{chrono::NaiveDateTime, Uuid},
    };

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CommentRow {
        pub id: Uuid,
        pub text: String,
        pub article_id: Uuid,
        pub author_id: Uuid,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }
}
