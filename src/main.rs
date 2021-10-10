//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::Result;
use std::env;
use storage::Storage;

// use mysql_async::prelude::*;

mod dockerhub;
mod storage;

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("DSN")?;
    let pool = mysql_async::Pool::new(&url[..]);

    let s = storage::MysqlStorage::new(pool);
    match s.projects().await {
        Ok(rs) => println!("result: {:?}", rs),
        Err(e) => eprintln!("error: {}", e),
    }

    s.update_gh_l8st_rel(99465516409683968, "v2.2.2".to_owned())
        .await?;

    s.update_dh_l8st_tag(99465516409683968, "v2.2.2".to_owned())
        .await?;

    s.disconnect().await?;

    // DockerHub
    tokio::task::spawn_blocking(move || {
        if let Ok(repos) = dockerhub::Collection::<dockerhub::Repository>::of("library") {
            for repo in repos {
                match repo {
                    Ok(r) => println!("repository: {}", r.name),
                    Err(e) => eprintln!("error: {}", e),
                }
            }
        }
    })
    .await?;

    Ok(())
}
