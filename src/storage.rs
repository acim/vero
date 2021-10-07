use std::fmt::Error;
use mysql_async::prelude::*;
use async_trait::async_trait;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Project {
    id: u64,
}

#[async_trait]
pub trait Storage {
    async fn projects(&self) -> Result<Vec<Project>, Error>;
    async fn update_gh_l8st_rel(id: u64, version: &str) -> Result<(), Error>;
    async fn update_dh_l8st_tag(id: u64, tag: &str) -> Result<(), Error>;
}

pub struct MysqlStorage {
    pool: mysql_async::Pool,
}

#[async_trait]
impl Storage for MysqlStorage {
    async fn projects(&self) -> Result<Vec<Project>, Error> {
        let mut conn = self.pool.get_conn().await?;
        let q = r"SELECT id, gh_owner, gh_repo, gh_l8st_rel, dh_owner, dh_repo, dockerhub_l8st_tag
                  FROM projects";
        let projects = conn.exec(q, ());
        todo!()
    }

    async fn update_gh_l8st_rel(id: u64, version: &str) -> Result<(), Error> {
        todo!()
    }

    async fn update_dh_l8st_tag(id: u64, tag: &str) -> Result<(), Error> {
        todo!()
    }
}
