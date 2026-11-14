use ::graphql::{schema, BlogSchema};
use actix_cors::Cors;
use actix_web::{guard, http, middleware, web, App, HttpResponse, HttpServer, Result};
use async_graphql_actix_web;
use std::io;
use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};
use env_logger;

async fn handle_graphql(schema: web::Data<BlogSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

fn setup_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .init()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    setup_logger();

    let schema = schema();

    let result = HttpServer::new(move || {
        let cors = Cors::permissive();
        //.allowed_origin("http://localhost:5000")
        //.allowed_origin("http://localhost:5010")
        //.allowed_origin("http://localhost:3000");
        App::new()
            .data(schema.clone())
            .wrap(cors)
            .service(web::resource("/").guard(guard::Post()).to(handle_graphql))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind(format!("0.0.0.0:5000"))?
    .run()
    .await;
    result
}
