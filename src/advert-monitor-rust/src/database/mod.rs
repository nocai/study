pub mod plan;

use std::env;

use async_std::task::block_on;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

lazy_static! {
    pub static ref MYSQLPOOL: Pool<MySql> = mysqlpool();
}

fn mysqlpool() -> Pool<MySql> {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&database_url);

    block_on(pool).unwrap()
}
