//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::Result;
use clap::{ColorChoice, Parser};
use semver::Version;
use std::env;
use storage::Storage;

// use mysql_async::prelude::*;

mod dockerhub;
mod storage;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Parser, Debug)]
#[clap(color = ColorChoice::Auto)]
struct Opts {
    #[clap(short, long, value_parser, default_value = "default.conf")]
    config: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Import(ImportCmd),
    Serve(ServeCmd),
}

/// Import data
#[derive(Parser, Debug)]
struct ImportCmd {
    #[clap(subcommand)]
    subcmd: SubSubCommand,
}

/// Run server
#[derive(Parser, Debug)]
struct ServeCmd {}

#[derive(Parser, Debug)]
enum SubSubCommand {
    DockerHub(DockerHubCmd),
}

/// Import data from DockerHub
#[derive(Parser, Debug)]
struct DockerHubCmd {}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);

    let url = env::var("DSN").unwrap();
    let pool = mysql_async::Pool::new(&url[..]);

    let s = storage::MysqlStorage::new(pool);

    match opts.subcmd {
        SubCommand::Import(sc) => match sc.subcmd {
            SubSubCommand::DockerHub(_dh) => {
                let c = dockerhub::Client::new();
                for r in c.repos("library".to_string()).await.unwrap() {
                    if r.description.starts_with("DEPRECATED") {
                        continue;
                    }
                    println!("repository: {}", r.name);
                    s.insert_dh("library".to_string(), r.name).await.unwrap();
                }
            }
        },
        SubCommand::Serve(_s) => {
            let c = dockerhub::Client::new();
            for r in s.projects().await.unwrap() {
                let owner = r.dh_owner.unwrap();
                let repo = r.dh_repo.unwrap();
                println!("owner: {owner:?} repo: {repo:?}");
                let latest = c.latest(owner, repo).await.unwrap();
                match r.dh_l8st_tag {
                    Some(t) => {
                        let tt = Version::parse(&t[..]).unwrap();
                        if latest > tt {
                            let _ = s
                                .update_dh_l8st_tag(r.id, latest.to_string())
                                .await
                                .unwrap();
                        }
                    }
                    None => {
                        let _ = s
                            .update_dh_l8st_tag(r.id, latest.to_string())
                            .await
                            .unwrap();
                    }
                }
                // let db_latest =
            }
            // s.update_gh_l8st_rel(99465516409683968, "v2.2.2".to_owned())
            //     .await
            //     .unwrap();

            // s.update_dh_l8st_tag(99465516409683968, "v2.2.2".to_owned())
            //     .await
            //     .unwrap();
        }
    }

    s.disconnect().await.unwrap();

    Ok(())
}
