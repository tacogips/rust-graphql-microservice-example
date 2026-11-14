use anyhow::{anyhow, Result};
use log;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{
    self,
    database::Database,
    postgres::{PgPool, PgPoolOptions},
    query, query_as,
    types::{
        chrono::{DateTime, NaiveDateTime, Utc},
        Uuid,
    },
    Error as SqlError, Pool, Postgres,
};

use async_trait::async_trait;

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(rename = "article_status", rename_all = "lowercase")]
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

#[async_trait]
pub trait ArticleService {
    async fn find_articles(&self, overview_text: bool) -> Result<Vec<ArticleRow>>;
    async fn get_article(&self, article_id: Uuid) -> Result<Option<ArticleRow>>;
}

#[derive(Debug)]
pub struct ArticleServicePg {
    db_pool: PgPool,
}

impl ArticleServicePg {
    pub fn new(db_pool: PgPool) -> ArticleServicePg {
        ArticleServicePg { db_pool }
    }
}

#[async_trait]
impl ArticleService for ArticleServicePg {
    async fn find_articles(&self, overview_text: bool) -> Result<Vec<ArticleRow>> {
        //TODO(tacogips) findout more concise way to branch query generation
        let query_result = if overview_text {
            query_as!(
                ArticleRow,
                r#"
            select
                id,
                status as "status: _",
                substring(text,10) as text,
                created_at,
                updated_at,
                author_id

            from article;"#
            )
            .fetch_all(&self.db_pool)
            .await
        } else {
            query_as!(
                ArticleRow,
                r#"
            select
                id,
                status as "status: _",
                 text,
                created_at,
                updated_at,
                author_id

            from article;"#
            )
            .fetch_all(&self.db_pool)
            .await
        };

        query_result.map_err(|e| anyhow!("find article error :{}", e))
    }

    async fn get_article(&self, article_id: Uuid) -> Result<Option<ArticleRow>> {
        //TODO(tacogips) findout more concise way to branch query generation
        let query_result = query_as!(
            ArticleRow,
            r#"
            select
                id,
                status as "status: _",
                text,
                created_at,
                updated_at,
                author_id
            from article where id = $1; "#,
            article_id,
        )
        .fetch_one(&self.db_pool)
        .await;

        match query_result {
            Ok(article) => Ok(Some(article)),
            Err(e) => match e {
                SqlError::RowNotFound => Ok(None),
                e => Err(anyhow!("find article error :{}", e)),
            },
        }
    }
}
