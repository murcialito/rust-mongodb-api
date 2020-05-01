use dotenv::dotenv;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use std::env;

pub type Pool = r2d2::Pool<MongodbConnectionManager>;

pub fn init_pool() -> Pool {
    dotenv().ok();
    let mongo_addr = env::var("MONGO_ADDR").expect("MONGO_ADDR must be set");
    let mongo_port = env::var("MONGO_PORT").expect("MONGO_PORT must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME env var must be set");
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&mongo_addr, mongo_port.parse::<u16>().unwrap())
            .with_db(&db_name)
            //.with_auth("root", "password")
            .build(),
    );
    match Pool::builder().max_size(64).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create mongodb pool {}", e),
    }
}