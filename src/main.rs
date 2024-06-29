extern crate dotenv;
mod app;
mod model;

use app::builder::AppState;
use dotenv::dotenv;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use actix_web::HttpServer;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let connect_options = ConnectOptions::new(std::env::var("DATABASE_URL").unwrap()).to_owned();

    let db = Database::connect(connect_options).await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    let state = AppState { conn: db };

    HttpServer::new(move || app::builder::build_app(state.clone()))
        .bind(("127.0.0.1", std::env::var("PORT").unwrap().parse().unwrap()))?
        .run()
        .await
}
