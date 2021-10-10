//!
use async_trait::async_trait;
use failure::Fail;
use mysql_async::prelude::*;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Project {
    id: u64,
    gh_owner: Option<String>,
    gh_repo: Option<String>,
    gh_l8st_rel: Option<String>,
    dh_owner: Option<String>,
    dh_repo: Option<String>,
    dh_l8st_tag: Option<String>,
}

#[derive(Debug, Fail)]
#[fail(display = "storage error {}", m)]
pub struct Error {
    m: String,
}

impl From<mysql_async::Error> for Error {
    fn from(err: mysql_async::Error) -> Error {
        Error { m: err.to_string() }
    }
}

#[async_trait]
pub trait Storage {
    async fn projects(&self) -> Result<Vec<Project>, Error>;
    async fn update_gh_l8st_rel(&self, id: u64, ver: String) -> tokio::task::JoinHandle<()>;
    async fn update_dh_l8st_tag(&self, id: u64, tag: &str) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct MysqlStorage {
    pool: mysql_async::Pool,
}

impl MysqlStorage {
    pub fn new(pool: mysql_async::Pool) -> Self {
        Self { pool }
    }

    pub async fn disconnect(&self) -> Result<(), Error> {
        let rs = self.pool.clone().disconnect();
        Ok(rs.await?)
    }
}

#[async_trait]
impl Storage for MysqlStorage {
    async fn projects(&self) -> Result<Vec<Project>, Error> {
        let mut conn = self.pool.get_conn().await?;
        let q = r"SELECT id, gh_owner, gh_repo, gh_l8st_rel, dh_owner, dh_repo, dh_l8st_tag
                  FROM projects";
        let rs = conn.exec_map(
            q,
            (),
            |(id, gh_owner, gh_repo, gh_l8st_rel, dh_owner, dh_repo, dh_l8st_tag)| Project {
                id,
                gh_owner,
                gh_repo,
                gh_l8st_rel,
                dh_owner,
                dh_repo,
                dh_l8st_tag,
            },
        );
        Ok(rs.await?)
    }

    async fn update_gh_l8st_rel(&self, id: u64, ver: String) -> tokio::task::JoinHandle<()> {
        let mut conn = self.pool.get_conn().await.unwrap();
        let q = r"UPDATE projects SET gh_l8st_rel=:gh_l8st_rel WHERE id=:id";

        tokio::spawn(async move {
            match conn
                .exec_drop(
                    q,
                    params! {
                        "id" => id,
                        "gh_l8st_rel" => ver,
                    },
                )
                .await
            {
                Ok(_) => println!("update_gh_l8st_rel"),
                Err(e) => eprintln!("error: {}", e),
            }
        })
    }

    async fn update_dh_l8st_tag(&self, id: u64, tag: &str) -> Result<(), Error> {
        let mut conn = self.pool.get_conn().await?;
        let q = r"UPDATE projects SET dh_l8st_tag=:dh_l8st_tag WHERE id=:id";
        let rs = conn.exec_drop(
            q,
            params! {
                "id" => id,
                "dh_l8st_tag" => tag,
            },
        );
        Ok(rs.await?)
    }
}
