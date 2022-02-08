pub mod plan;
pub mod supe;
pub mod dsp;

use std::env;

use async_std::task::block_on;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

lazy_static! {
    pub static ref DSP_POOL: Pool<MySql> = {
        let database_url = env::var("DATABASE_URL").unwrap();
        mysqlpool(&database_url)
    };
    pub static ref SUPE_POOL: Pool<MySql> = {
        let database_url = env::var("DATABASE_URL_SUPE").unwrap();
        mysqlpool(&database_url)
    };
}

fn mysqlpool(url: &str) -> Pool<MySql> {
    let pool = MySqlPoolOptions::new().max_connections(1).connect(url);

    block_on(pool).unwrap()
}
