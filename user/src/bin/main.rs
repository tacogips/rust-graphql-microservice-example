use ::user::service::*;

use anyhow::anyhow;
use couchbase::Cluster;
use env_logger;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::io;

use std::str::FromStr;

use uuid::Uuid;

use actix_web::{get, middleware, put, web, App, Error, HttpResponse, HttpServer};

use std::sync::Arc;

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
    let service = UserServiceCouch::new(data.db_cluster.clone());
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
    let service = UserServiceCouch::new(data.db_cluster.clone());

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

#[put("/default/users")]
async fn create_default_user(data: web::Data<SharedData>) -> Result<HttpResponse, Error> {
    let service = UserServiceCouch::new(data.db_cluster.clone());

    match service.create_default_users().await {
        Ok(()) => Ok(HttpResponse::Ok().json(json!({"message":"ok"}))),
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
    db_cluster: Arc<Cluster>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    setup_logger();

    let db_conn_str =
        env_value!("DATABASE_URL").unwrap_or_else(|_| "couchbase://127.0.0.1".to_string());

    let db_user = env_value!("DB_USER").unwrap_or_else(|_| "Administrator".to_string());
    let db_pass = env_value!("DB_PASS").unwrap_or_else(|_| "password".to_string());
    //
    //let db_user = env_value!("DB_USER").unwrap_or_else(|_| "sample_user".to_string());
    //let db_pass = env_value!("DB_PASS").unwrap_or_else(|_| "pass123".to_string());

    println!("##### {},{}", db_user, db_pass);

    let db_cluster = Arc::new(Cluster::connect(&db_conn_str, &db_user, &db_pass));

    log::info!("db connection {}", db_conn_str);

    let data = SharedData { db_cluster };
    log::info!("user server is listening at 5000...");

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(find_users)
            .service(get_user)
            .service(create_default_user)
    })
    .bind(format!("0.0.0.0:5000"))?
    .run()
    .await
}
