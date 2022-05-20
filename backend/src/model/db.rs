// https://docs.rs/sqlx/latest/sqlx/pool/index.html
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;
use std::time::Duration;

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";

pub type Db = Pool<Postgres>;

pub async fn init() {
    new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await;
}

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let con_str = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);

    PgPoolOptions::new()
        .max_connections(max_con)
        .connect_timeout(Duration::from_millis(500))
        .connect(&con_str)
        .await
}
