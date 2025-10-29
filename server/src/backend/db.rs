use diesel::mysql::MysqlConnection;
use diesel::{prelude::*, r2d2};
use std::env;

embed_migrations!();

pub type DatabasePool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_pool() -> DatabasePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn run_migrations() {
    let connection = &mut establish_connection();
    embedded_migrations::run(connection).expect("Failed to run migrations");
}
