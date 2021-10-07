//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use storage::Storage;

// use mysql_async::prelude::*;

mod storage;

#[tokio::main]
async fn main() {
    let s = storage::MysqlStorage::new(mysql_async::Pool::new(""));
    let ps = s.projects().await.unwrap();
    println!("result {:?}", ps);
}
