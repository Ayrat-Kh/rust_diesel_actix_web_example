extern crate diesel;
extern crate dotenv;

use actix_web::{middleware, web, App, HttpServer};
use app::tweets::routes::register_tweets;
use dotenv::dotenv;

pub mod app;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    println!("Db init");

    let db_pool = crate::app::db::get_connection_pool();

    println!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .configure(register_tweets)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
