use anyhow::{anyhow, Result};

use uuid::Uuid;

use chrono::{NaiveDateTime, Utc};
use couchbase::{
    BucketSettings, BucketSettingsBuilder, Cluster, CouchbaseError, CreateBucketOptions,
    GetBucketOptions, GetOptions, PingOptions, QueryOptions, UpsertOptions,
};
use futures::stream::StreamExt;
use log;
use models::user::*;
use serde_json::json;
use std::result::Result as StdResult;
use tokio;

use async_trait::async_trait;
use serde::{self, Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

#[async_trait]
pub trait UserService {
    async fn find_users(&self, filter_ids: Option<Vec<String>>) -> Result<Vec<UserRow>>;
    async fn get_user(&self, user_id: Uuid) -> Result<Option<UserRow>>;
    async fn create_default_users(&self) -> Result<()>;
}

pub struct UserServiceCouch {
    db_cluster: Arc<Cluster>,
}

impl UserServiceCouch {
    pub fn new(db_cluster: Arc<Cluster>) -> Self {
        Self { db_cluster }
    }
}
const USER_BUCKET_NAME: &str = "user-bucket";

#[async_trait]
impl UserService for UserServiceCouch {
    async fn find_users(&self, filter_ids: Option<Vec<String>>) -> Result<Vec<UserRow>> {
        //TODO(tacogips) you need to create inedex before execute this method
        //CREATE PRIMARY INDEX `user-bucket-index` ON `user-bucket` USING GSI;
        //
        //TODO(taocgips) ignoring the filter_ids now

        match self
            .db_cluster
            .query("select * from `user-bucket`", QueryOptions::default())
            .await
        {
            Ok(mut result) => {
                let mut user_rows = Vec::<serde_json::Value>::new();
                for row in result.rows::<serde_json::Value>().next().await {
                    let obj = row.unwrap();
                    user_rows.push(obj);
                }

                // result of  query above must be below
                //
                //[
                //  {
                //    "user-bucket": {
                //      "created_at": "2021-05-12T10:02:27.014474831",
                //      "email": "",
                //      "id": "25f8ea00-52f4-4c58-b8db-8b7075982139",
                //      "name": "user_2",
                //      "updated_at": "2021-05-12T10:02:27.014475655"
                //    }
                //  },
                //  {
                //    "user-bucket": {
                //      "created_at": "2021-05-12T10:02:27.013928428",
                //      "email": "",
                //      "id": "bccdd9f9-c2e8-49dc-9ba1-5cc992d8ada2",
                //      "name": "user_1",
                //      "updated_at": "2021-05-12T10:02:27.013930624"
                //    }
                //  }
                //]
                //TODO(tacogips) convert json value to user
                Ok(vec![])
            }
            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    async fn get_user(&self, user_id: Uuid) -> Result<Option<UserRow>> {
        let user_bucket = self.db_cluster.bucket(USER_BUCKET_NAME);

        let coll = user_bucket.default_collection();
        match coll.get(user_id.to_string(), GetOptions::default()).await {
            Ok(d) => {
                let user: UserRow = d.content()?;
                Ok(Some(user))
            }
            Err(CouchbaseError::DocumentNotFound { ctx: _ }) => Ok(None),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    async fn create_default_users(&self) -> Result<()> {
        let settings = BucketSettingsBuilder::new(USER_BUCKET_NAME)
            .flush_enabled(true)
            .build();

        let buckets = self.db_cluster.buckets();
        if let Err(e) = buckets
            .create_bucket(settings, CreateBucketOptions::default())
            .await
        {
            log::warn!("{}", e)
        }

        // wait bucket creation finished
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        let user_bucket = self.db_cluster.bucket(USER_BUCKET_NAME);
        let coll = user_bucket.default_collection();

        let user1 = UserRow {
            id: Uuid::from_str("bccdd9f9-c2e8-49dc-9ba1-5cc992d8ada2").unwrap(),
            name: "user_1".to_string(),
            email: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        coll.upsert(
            user1.id.clone().to_string(),
            user1,
            UpsertOptions::default(),
        )
        .await?;

        let user2 = UserRow {
            id: Uuid::from_str("25f8ea00-52f4-4c58-b8db-8b7075982139").unwrap(),
            name: "user_2".to_string(),
            email: "".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        coll.upsert(
            user2.id.clone().to_string(),
            user2,
            UpsertOptions::default(),
        )
        .await?;
        //TODO(tacogips) run create index
        //CREATE PRIMARY INDEX `user-bucket-index` ON `user-bucket` USING GSI;
        Ok(())
    }
}
