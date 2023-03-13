use actix_web::{web, App, HttpServer, middleware};

mod controller;
mod db;
mod repository;

use crate::controller::pokemon_controller::get_pokemon_by_id;
use crate::db::Pool;
use r2d2_sqlite::{self, SqliteConnectionManager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // connect to SQLite DB
    let manager = SqliteConnectionManager::file("pokemon.db");
    let pool = Pool::new(manager).unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_pokemon_by_id)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}