//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use std::env;
use storage::Storage;

// use mysql_async::prelude::*;

mod storage;

#[tokio::main]
async fn main() {
    let url = env::var("DSN").unwrap();
    let pool = mysql_async::Pool::new(&url[..]);

    let s = storage::MysqlStorage::new(pool);
    match s.projects().await {
        Ok(rs) => println!("result: {:?}", rs),
        Err(e) => eprintln!("error: {}", e),
    }

    s.disconnect().await.unwrap()
}
