mod models;
mod db;
mod cache;
mod routes;

use actix_web::{web, App, HttpServer};
use db::Db;
use cache::Cache;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let db = Db::new(&database_url).await.unwrap();
    let cache = Cache::new(&redis_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(cache.clone()))
            .route("/users", web::post().to(routes::create_user))
            .route("/users/{id}", web::get().to(routes::get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

