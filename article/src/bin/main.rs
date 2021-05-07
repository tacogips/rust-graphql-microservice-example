use ::article::service::*;

use anyhow::anyhow;
use env_logger;
use log;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::io;

use sqlx::types::Uuid;
use std::str::FromStr;

use actix_web::{delete, get, middleware, web, App, Error, HttpResponse, HttpServer};
use serde_json::json;
use std::time::Duration;

macro_rules! env_value {
    ($env_key:expr) => {
        env::var($env_key).map_err(|e| anyhow!("env {} not found. Err:{:?}", $env_key, e))
    };
}
#[get("/articles")]
async fn find_articles(data: web::Data<SharedData>) -> Result<HttpResponse, Error> {
    let service = ArticleServicePg::new(data.db_pool.clone());
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
    let service = ArticleServicePg::new(data.db_pool.clone());

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

#[delete("/article/{id}")]
async fn delete_article(
    data: web::Data<SharedData>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let service = ArticleServicePg::new(data.db_pool.clone());

    let article_id = path.into_inner();

    match service.delete_article(&article_id).await {
        Ok(()) => Ok(HttpResponse::Ok().json(json!({"message":"ok"}))),
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
        .unwrap_or_else(|_| "postgres://db_user:pass@localhost:5432/article_db".to_string());

    log::info!("db connection {}", db_conn_str);

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect(&db_conn_str)
        .await
        .expect(&format!("faield to connect db {}", db_conn_str));

    let data = SharedData { db_pool };
    log::info!("article server is listening at 5000...");

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(find_articles)
            .service(get_article)
            .service(delete_article)
    })
    .bind(format!("0.0.0.0:5000"))?
    .run()
    .await
}
