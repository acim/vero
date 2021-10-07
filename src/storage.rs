use mysql_async::prelude::*;
use async_trait::async_trait;
use failure::Fail;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Project {
    id: u64,
    gh_owner: String,
    gh_repo: String,
    gh_l8st_rel: String,
    dh_owner: String,
     dh_repo: String,
      dh_l8st_tag: String,
    }

#[derive(Debug, Fail)]
#[fail(display = "storage error {}", m)]
pub struct Error {
    m: String,
}

impl From<mysql_async::Error> for Error {
    fn from(err: mysql_async::Error) -> Error {
        Error{m: err.to_string()}
    }
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

impl MysqlStorage {
    pub fn new(pool: mysql_async::Pool) -> Self {
        Self{pool}
    }

    pub async fn disconnect(&self) -> Result<(), Error>{
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
        let rs = conn.exec_map(q, (),|(id, gh_owner, gh_repo, gh_l8st_rel, dh_owner, dh_repo, dh_l8st_tag)| Project { id, gh_owner, gh_repo, gh_l8st_rel, dh_owner, dh_repo, dh_l8st_tag });
Ok(rs.await?)
    }

    async fn update_gh_l8st_rel(id: u64, version: &str) -> Result<(), Error> {
        todo!()
    }

    async fn update_dh_l8st_tag(id: u64, tag: &str) -> Result<(), Error> {
        todo!()
    }
}
