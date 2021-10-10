//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::Result;
use clap::{AppSettings, Clap};
use std::env;
use storage::Storage;

// use mysql_async::prelude::*;

mod dockerhub;
mod storage;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    // input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Import(ImportCmd),
    Server(ServerCmd),
}

/// Import data
#[derive(Clap)]
struct ImportCmd {
    /// Print debug info
    #[clap(short)]
    debug: bool,
    #[clap(subcommand)]
    subcmd: SubSubCommand,
}

/// Run server
#[derive(Clap)]
struct ServerCmd {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

#[derive(Clap)]
enum SubSubCommand {
    DockerHub(DockerHubCmd),
}

/// Import data from DockerHub
#[derive(Clap)]
struct DockerHubCmd {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    let url = env::var("DSN")?;
    let pool = mysql_async::Pool::new(&url[..]);

    let s = storage::MysqlStorage::new(pool);
    match s.projects().await {
        Ok(rs) => println!("result: {:?}", rs),
        Err(e) => eprintln!("error: {}", e),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Import(sc) => match sc.subcmd {
            SubSubCommand::DockerHub(_dh) => {
                println!(">>>>>>>>>>>>>>>>>>>>>>>>>>> Import from DockerHub");
                tokio::task::spawn_blocking(move || {
                    if let Ok(repos) = dockerhub::Collection::<dockerhub::Repository>::of("library")
                    {
                        for repo in repos {
                            match repo {
                                Ok(r) => {
                                    println!("repository: {}", r.name);
                                    s.upsert_dh("library".to_string(), r.name).await?;

                                    return Ok(());
                                }
                                Err(e) => eprintln!("error: {}", e),
                            }
                        }
                    }
                })
                .await?;

                return Ok(());
            }
        },
        SubCommand::Server(_s) => {
            s.update_gh_l8st_rel(99465516409683968, "v2.2.2".to_owned())
                .await?;

            s.update_dh_l8st_tag(99465516409683968, "v2.2.2".to_owned())
                .await?;

            s.disconnect().await?;

            Ok(())
        }
    }
}
