use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DbPool {
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
