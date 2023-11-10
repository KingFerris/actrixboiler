use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod routes;
mod models;
mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(web::resource("/get").to(routes::get_posts))
        .service(web::resource("/post").to(routes::create_post))
        .service(web::resource("/update/{id}").to(routes::update_post))
        .service(web::resource("/delete/{title}").to(routes::delete_post))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}