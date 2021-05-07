use anyhow::{anyhow, Result};
use models::article::*;
use sqlx::{self, postgres::PgPool, query_as, types::Uuid, Error as SqlError};

use async_trait::async_trait;
use std::str::FromStr;

#[async_trait]
pub trait ArticleService {
    async fn find_articles(&self, overview_text: bool) -> Result<Vec<ArticleRow>>;
    async fn get_article(&self, article_id: Uuid) -> Result<Option<ArticleRow>>;
    async fn delete_article(&self, article_id: &str) -> Result<()>;
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

    async fn delete_article(&self, article_id: &str) -> Result<()> {
        let article_id_as_uuid: Uuid = Uuid::from_str(article_id)?;

        let mut tx = self.db_pool.begin().await?;

        query_as!(
            CommentRow,
            r#"delete from article where id = $1;"#,
            &article_id_as_uuid
        )
        .bind("")
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
