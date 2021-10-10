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
    async fn update_gh_l8st_rel(&self, id: u64, ver: String) -> Result<u64>;
    async fn update_dh_l8st_tag(&self, id: u64, tag: String) -> Result<u64>;
    async fn insert_dh(&self, owner: String, repo: String) -> Result<u64>;
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
        self.pool.clone().disconnect().await?;
        Ok(())
    }
}

#[async_trait]
impl Storage for MysqlStorage {
    async fn projects(&self) -> Result<Vec<Project>> {
        let q = r"SELECT id, gh_owner, gh_repo, gh_l8st_rel, dh_owner, dh_repo, dh_l8st_tag
        FROM projects";

        let mut conn = self.pool.get_conn().await?;

        let res = conn
            .exec_map(
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
            )
            .await?;

        Ok(res)
    }

    async fn update_gh_l8st_rel(&self, id: u64, ver: String) -> Result<u64> {
        let q = r"UPDATE projects SET gh_l8st_rel=:ver WHERE id=:id";

        let mut conn = self.pool.get_conn().await?;

        conn.exec_drop(
            q,
            params! {
                "id" => id,
                "ver" => ver,
            },
        )
        .await?;

        Ok(conn.affected_rows())
    }

    async fn update_dh_l8st_tag(&self, id: u64, tag: String) -> Result<u64> {
        let q = r"UPDATE projects SET dh_l8st_tag=:tag WHERE id=:id";

        let mut conn = self.pool.get_conn().await?;

        conn.exec_drop(
            q,
            params! {
                "id" => id,
                "tag" => tag,
            },
        )
        .await?;

        Ok(conn.affected_rows())
    }

    async fn insert_dh(&self, owner: String, repo: String) -> Result<u64> {
        let q = r"INSERT IGNORE INTO projects (dh_owner, dh_repo) VALUES(:owner, :repo)";

        let mut conn = self.pool.get_conn().await?;

        conn.exec_drop(
            q,
            params! {
                "owner" => owner,
                "repo" => repo,
            },
        )
        .await?;

        Ok(conn.affected_rows())
    }
}
