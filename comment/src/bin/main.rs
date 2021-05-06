use ::comment::service::*;

use anyhow::{anyhow, Error as AnyError};
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    Pool, Postgres,
};
use std::env;
use std::io;

use sqlx::types::Uuid;
use std::str::FromStr;

use actix_web::{
    get,
    http::StatusCode,
    middleware, post, web,
    web::{Json, Query},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};

use std::time::Duration;

macro_rules! env_value {
    ($env_key:expr) => {
        env::var($env_key).map_err(|e| anyhow!("env {} not found. Err:{:?}", $env_key, e))
    };
}

#[derive(Deserialize, Serialize)]
pub struct FindCommentQuery {
    article_id: String,
}

#[get("/comments")]
async fn find_comments(
    data: web::Data<SharedData>,
    query: web::Query<FindCommentQuery>,
) -> Result<HttpResponse, Error> {
    let service = CommentServicePg::new(data.db_pool.clone());
    match service.find_comments(&query.article_id).await {
        Ok(comments) => Ok(HttpResponse::Ok().json(comments)),
        Err(e) => {
            log::error!("find comment {:?}", e);
            Err(Error::from(()))
        }
    }
}

fn setup_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .init()
}

#[derive(Clone)]
pub struct SharedData {
    db_pool: PgPool,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    setup_logger();

    let db_conn_str = env_value!("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://db_comment:pass@localhost:5432/comment_db".to_string());

    log::info!("db connection {}", db_conn_str);

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect(&db_conn_str)
        .await
        .expect(&format!("faield to connect db {}", db_conn_str));

    let data = SharedData { db_pool };
    log::info!("comment server is listening at 5000...");

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(find_comments)
    })
    .bind(format!("0.0.0.0:5000"))?
    .run()
    .await
}
