use ::user::service::*;

use anyhow::anyhow;
use env_logger;
use log;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::io;

use sqlx::types::Uuid;
use std::str::FromStr;

use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};

use std::time::Duration;

macro_rules! env_value {
    ($env_key:expr) => {
        env::var($env_key).map_err(|e| anyhow!("env {} not found. Err:{:?}", $env_key, e))
    };
}

#[derive(Deserialize, Serialize)]
pub struct FindUserQuery {
    ids: Option<Vec<String>>,
}

#[get("/users")]
async fn find_users(
    data: web::Data<SharedData>,
    query: web::Query<FindUserQuery>,
) -> Result<HttpResponse, Error> {
    let service = UserServicePg::new(data.db_pool.clone());
    match service.find_users(query.ids.clone()).await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            log::error!("find user {:?}", e);
            Err(Error::from(()))
        }
    }
}

#[get("/user/{id}")]
async fn get_user(
    data: web::Data<SharedData>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let service = UserServicePg::new(data.db_pool.clone());

    let user_id = path.into_inner().0;
    //Uuid
    let user_id = match Uuid::from_str(&user_id) {
        Ok(id) => id,
        Err(e) => {
            log::error!("{}", e);
            return Err(Error::from(()));
        }
    };

    match service.get_user(user_id).await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            log::error!("find user {:?}", e);
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
        .unwrap_or_else(|_| "postgres://db_user:pass@localhost:5432/user_db".to_string());

    log::info!("db connection {}", db_conn_str);

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect(&db_conn_str)
        .await
        .expect(&format!("faield to connect db {}", db_conn_str));

    let data = SharedData { db_pool };
    log::info!("user server is listening at 5000...");

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(find_users)
            .service(get_user)
    })
    .bind(format!("0.0.0.0:5000"))?
    .run()
    .await
}
