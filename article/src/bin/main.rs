use ::article::{request::*, service::*};

use anyhow::{anyhow, Error as AnyError};
use env_logger;
use log;
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

use serde::Deserialize;

macro_rules! env_value {
    ($env_key:expr) => {
        env::var($env_key).map_err(|e| anyhow!("env {} not found. Err:{:?}", $env_key, e))
    };
}
#[get("/articles")]
async fn find_articles(data: web::Data<SharedData>) -> Result<HttpResponse, Error> {
    let service = ArtilceServicePg::new(data.db_pool.clone());
    match service.find_articles(true).await {
        Ok(articles) => Ok(HttpResponse::Ok().json(articles)),
        Err(e) => {
            log::error!("find article {:?}", e);
            Err(Error::from(()))
        }
    }
}

#[get("/article/{id}")]
async fn get_article(
    data: web::Data<SharedData>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let service = ArtilceServicePg::new(data.db_pool.clone());

    let article_id = path.into_inner();
    //Uuid
    let article_id = match Uuid::from_str(&article_id) {
        Ok(id) => id,
        Err(e) => {
            log::error!("{}", e);
            return Err(Error::from(()));
        }
    };

    match service.get_article(article_id).await {
        Ok(articles) => Ok(HttpResponse::Ok().json(articles)),
        Err(e) => {
            log::error!("find article {:?}", e);
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
        .unwrap_or_else(|e| "postgres://db_user:pass@localhost:5432/article_db".to_string());

    log::info!("db connection {}", db_conn_str);

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn_str)
        .await
        .unwrap();

    let data = SharedData { db_pool };
    log::info!("article server is listening at 5000...");

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(find_articles)
    })
    .bind(format!("0.0.0.0:5000"))?
    .run()
    .await
}
