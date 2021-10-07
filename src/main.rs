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

    let mut handles = vec![];

    handles.push(
        s.update_gh_l8st_rel(99465516409683968, "v1.0.0".to_owned())
            .await,
    );

    {
        let s = s.clone();
        handles.push(tokio::spawn(async move {
            match s.update_dh_l8st_tag(99465516409683968, "v1.0.0").await {
                Ok(_) => (),
                Err(e) => eprintln!("error: {}", e),
            }
        }));
    }

    futures::future::join_all(handles).await;

    s.disconnect().await.unwrap();
}
