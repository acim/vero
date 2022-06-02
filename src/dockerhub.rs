//!
use reqwest::Result;
use semver::{BuildMetadata, Prerelease, Version};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new(),
        }
    }

    pub async fn repos(&self, owner: String) -> Result<Vec<Repository>> {
        let mut url = format!(
            "https://hub.docker.com/v2/repositories/{}/?page={}&page_size={}",
            owner, 1, 100
        );

        let mut results = vec![];

        loop {
            let mut res = self.get::<Repository>(url).await?;

            results.append(&mut res.results);

            match res.next {
                serde_json::Value::String(u) => url = u,
                _ => break,
            }
        }

        Ok(results)
    }

    pub async fn latest(&self, owner: String, repo: String) -> Result<Version> {
        let url = format!(
            "https://hub.docker.com/v2/repositories/{}/{}/tags/?page={}&page_size={}",
            owner, repo, 1, 100
        );

        let res = self.get::<Tag>(url).await?;

        let mut latest = Version::new(0, 0, 0);
        for tag in res.results {
            if let Ok(version) = Version::parse(&tag.name[..]) {
                if version > latest {
                    latest = version
                }
            }
        }

        latest.pre = Prerelease::EMPTY;
        latest.build = BuildMetadata::EMPTY;

        Ok(latest)
    }

    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<Response<T>> {
        println!("get url: {}", url);

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Response<T>>()
            .await?;

        Ok(res)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response<T> {
    pub count: u64,
    pub next: ::serde_json::Value,
    pub previous: ::serde_json::Value,
    pub results: Vec<T>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repository {
    pub user: String,
    pub name: String,
    pub namespace: String,
    pub repository_type: ::serde_json::Value,
    pub status: u64,
    pub description: String,
    pub is_private: bool,
    pub is_automated: bool,
    pub can_edit: bool,
    pub star_count: u64,
    pub pull_count: u64,
    pub last_updated: String,
    pub is_migrated: bool,
    pub collaborator_count: u64,
    pub affiliation: ::serde_json::Value,
    pub hub_user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub creator: u64,
    pub id: u64,
    pub image_id: ::serde_json::Value,
    pub images: Vec<Image>,
    pub last_updated: Option<String>,
    pub last_updater: u64,
    pub last_updater_username: String,
    pub name: String,
    pub repository: u64,
    pub full_size: u64,
    pub v2: bool,
    pub tag_status: String,
    pub tag_last_pulled: Option<String>,
    pub tag_last_pushed: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub architecture: String,
    pub features: String,
    pub variant: Option<String>,
    pub digest: Option<String>,
    pub os: String,
    pub os_features: String,
    pub os_version: ::serde_json::Value,
    pub size: u64,
    pub status: String,
    pub last_pulled: Option<String>,
    pub last_pushed: Option<String>,
}
