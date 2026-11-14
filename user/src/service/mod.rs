use anyhow::{anyhow, Result};
use log;
use models::user::*;
use serde::{Deserialize, Serialize};
use sqlx::{
    self,
    postgres::PgPool,
    query_as,
    types::{chrono::NaiveDateTime, uuid, Uuid},
    Error as SqlError,
};
use std::result::Result as StdResult;

use async_trait::async_trait;
use std::str::FromStr;

#[async_trait]
pub trait UserService {
    async fn find_users(&self, filter_ids: Option<Vec<String>>) -> Result<Vec<UserRow>>;
    async fn get_user(&self, user_id: Uuid) -> Result<Option<UserRow>>;
}

#[derive(Debug)]
pub struct UserServicePg {
    db_pool: PgPool,
}

impl UserServicePg {
    pub fn new(db_pool: PgPool) -> UserServicePg {
        UserServicePg { db_pool }
    }
}

#[async_trait]
impl UserService for UserServicePg {
    async fn find_users(&self, filter_ids: Option<Vec<String>>) -> Result<Vec<UserRow>> {
        //TODO(tacogips) findout more concise way to branch query generation
        let query_result = match filter_ids {
            Some(ids) => {
                // I used "any" clause to filter records with multiple id instead of "in" clause.
                // The issue about multiple parameters is here
                // https://www.reddit.com/r/rust/comments/ip4a0q/sql_x_how_do_you_parameterize_an_in_statement_or/

                let ids_as_uuid: StdResult<Vec<Uuid>, uuid::Error> = ids
                    .into_iter()
                    .map(|id| Uuid::from_str(&id))
                    .into_iter()
                    .collect();

                let ids_as_uuid = ids_as_uuid?;
                query_as!(
                    UserRow,
                    r#"select * from user_table where id = any($1);"#,
                    &ids_as_uuid
                )
                .fetch_all(&self.db_pool)
                .await
            }
            None => {
                query_as!(UserRow, r#" select * from user_table ;"#)
                    .fetch_all(&self.db_pool)
                    .await
            }
        };

        query_result.map_err(|e| anyhow!("find user error :{}", e))
    }

    async fn get_user(&self, user_id: Uuid) -> Result<Option<UserRow>> {
        //TODO(tacogips) findout more concise way to branch query generation
        let query_result = query_as!(
            UserRow,
            r#"
            select
                id,
                name,
                email,
                created_at,
                updated_at
            from user_table where id = $1; "#,
            user_id,
        )
        .fetch_one(&self.db_pool)
        .await;

        match query_result {
            Ok(user) => Ok(Some(user)),
            Err(e) => match e {
                SqlError::RowNotFound => Ok(None),
                e => Err(anyhow!("find user error :{}", e)),
            },
        }
    }
}
