use anyhow::{anyhow, Result};
use log;

use models::comment::*;

use sqlx::{self, postgres::PgPool, query_as, types::Uuid};

use async_trait::async_trait;
use std::str::FromStr;

#[async_trait]
pub trait CommentService {
    async fn find_comments(&self, article_id: &str) -> Result<Vec<CommentRow>>;
    async fn delete_comments(&self, article_id: &str) -> Result<()>;
}

#[derive(Debug)]
pub struct CommentServicePg {
    db_pool: PgPool,
}

impl CommentServicePg {
    pub fn new(db_pool: PgPool) -> CommentServicePg {
        CommentServicePg { db_pool }
    }
}

#[async_trait]
impl CommentService for CommentServicePg {
    async fn find_comments(&self, article_id: &str) -> Result<Vec<CommentRow>> {
        //TODO(tacogips) findout more concise way to branch query generation

        let article_id_as_uuid: Uuid = Uuid::from_str(article_id)?;
        let query_result = query_as!(
            CommentRow,
            r#"select * from comment_table where article_id = $1;"#,
            &article_id_as_uuid
        )
        .fetch_all(&self.db_pool)
        .await;

        query_result.map_err(|e| anyhow!("find comment error :{}", e))
    }

    async fn delete_comments(&self, article_id: &str) -> Result<()> {
        let article_id_as_uuid: Uuid = Uuid::from_str(article_id)?;

        let mut tx = self.db_pool.begin().await?;

        query_as!(
            CommentRow,
            r#"delete  from comment_table where article_id = $1;"#,
            &article_id_as_uuid
        )
        .bind("")
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
