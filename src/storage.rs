//!
use anyhow::Result;
use async_trait::async_trait;
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

#[async_trait]
pub trait Storage {
    async fn projects(&self) -> Result<Vec<Project>>;
    async fn update_gh_l8st_rel(&self, id: u64, ver: String) -> Result<()>;
    async fn update_dh_l8st_tag(&self, id: u64, tag: String) -> Result<()>;
}

#[derive(Clone)]
pub struct MysqlStorage {
    pool: mysql_async::Pool,
}

impl MysqlStorage {
    pub fn new(pool: mysql_async::Pool) -> Self {
        Self { pool }
    }

    pub async fn disconnect(&self) -> Result<()> {
        let rs = self.pool.clone().disconnect();
        Ok(rs.await?)
    }
}

#[async_trait]
impl Storage for MysqlStorage {
    async fn projects(&self) -> Result<Vec<Project>> {
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

    async fn update_gh_l8st_rel(&self, id: u64, ver: String) -> Result<()> {
        let mut conn = self.pool.get_conn().await.unwrap();
        let q = r"UPDATE projects SET gh_l8st_rel=:gh_l8st_rel WHERE id=:id";

        let rs = tokio::spawn(async move {
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
        });

        Ok(rs.await?)
    }

    async fn update_dh_l8st_tag(&self, id: u64, tag: String) -> Result<()> {
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
