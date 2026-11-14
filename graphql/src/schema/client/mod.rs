use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

use super::converter::*;

macro_rules! env_value {
    ($env_key:expr) => {
        env::var($env_key).map_err(|e| anyhow!("env {} not found. Err:{:?}", $env_key, e))
    };
}

lazy_static! {
    static ref CLIENT: Arc<reqwest::Client> = Arc::new(reqwest::Client::builder().build().unwrap());
    static ref ARTICLE_ENDPOINT: String = env_value!("ARTICLE_SERVICE_ENDPOINT").unwrap();
    static ref USER_ENDPOINT: String = env_value!("USER_SERVICE_ENDPOINT").unwrap();
    static ref COMMENT_ENDPOINT: String = env_value!("COMMENT_ENDPOINT").unwrap();
}

pub async fn find_articles_with_overview() -> Result<Vec<Article>> {
    let article_rows = CLIENT
        .get(format!("{}/articles", ARTICLE_ENDPOINT.as_str()))
        .send()
        .await?
        .json::<Vec<response::ArticleRow>>()
        .await?;

    Ok(Article::from(article_rows))
}

pub async fn get_article(article_id: &str) -> Result<Article> {
    let article_row = CLIENT
        .get(format!(
            "{}/article/{}",
            ARTICLE_ENDPOINT.as_str(),
            article_id
        ))
        .send()
        .await?
        .json::<response::ArticleRow>()
        .await?;
    Ok(article_row.into())
}

pub async fn find_users(user_ids: Vec<&str>) -> Result<Vec<User>> {
    let user_rows = CLIENT
        .get(format!("{}/users", USER_ENDPOINT.as_str()))
        .query(
            &user_ids
                .into_iter()
                .map(|id| ("ids[]", id))
                .collect::<Vec<(&str, &str)>>(),
        )
        .send()
        .await?
        .json::<Vec<response::UserRow>>()
        .await?;

    Ok(User::from(user_rows))
}

pub async fn get_user(user_id: &str) -> Result<User> {
    let user_row = CLIENT
        .get(format!("{}/user/{}", USER_ENDPOINT.as_str(), user_id))
        .send()
        .await?
        .json::<response::UserRow>()
        .await?;

    Ok(user_row.into())
}

pub async fn find_comments(article_id: &str) -> Result<Vec<Comment>> {
    let comment_rows = CLIENT
        .get(format!("{}/comments", COMMENT_ENDPOINT.as_str()))
        .query(&[("article_id", article_id)])
        .send()
        .await?
        .json::<Vec<response::CommentRow>>()
        .await?;

    Ok(Comment::from(comment_rows))
}
