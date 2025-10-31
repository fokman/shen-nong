use crate::config::Config;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

embed_migrations!();

pub type DatabasePool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlConnection {
    let config = Config::from_env();
    MysqlConnection::establish(config.database_url())
        .expect(&format!("Error connecting to {}", config.database_url()))
}

pub fn get_pool() -> DatabasePool {
    let config = Config::from_env();

    let manager =
        r2d2::ConnectionManager::<MysqlConnection>::new(config.database_url().to_string());
    r2d2::Pool::builder()
        .max_size(config.database.max_connections)
        .min_idle(Some(config.database.min_connections))
        .connection_timeout(std::time::Duration::from_secs(
            config.database.connect_timeout,
        ))
        .idle_timeout(Some(std::time::Duration::from_secs(
            config.database.idle_timeout,
        )))
        .build(manager)
        .expect("Failed to create pool")
}

pub fn run_migrations() {
    let connection = &mut establish_connection();
    embedded_migrations::run(connection).expect("Failed to run migrations");
}
