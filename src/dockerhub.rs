//!
use reqwest::Result;
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

    pub async fn latest(&self, owner: String, repo: String) -> Result<String> {
        let url = format!(
            "https://hub.docker.com/v2/repositories/{}/{}/tags/?page={}&page_size={}",
            owner, repo, 1, 100
        );

        let res = self.get::<Tag>(url).await?;

        for tag in res.results {
            println!("tag: {}", tag.name)
        }

        Ok("".to_string())
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub count: u64,
    pub next: ::serde_json::Value,
    pub previous: ::serde_json::Value,
    pub results: Vec<T>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub user: String,
    pub name: String,
    pub namespace: String,
    #[serde(rename = "repository_type")]
    pub repository_type: ::serde_json::Value,
    pub status: u64,
    pub description: String,
    #[serde(rename = "is_private")]
    pub is_private: bool,
    #[serde(rename = "is_automated")]
    pub is_automated: bool,
    #[serde(rename = "can_edit")]
    pub can_edit: bool,
    #[serde(rename = "star_count")]
    pub star_count: u64,
    #[serde(rename = "pull_count")]
    pub pull_count: u64,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "is_migrated")]
    pub is_migrated: bool,
    #[serde(rename = "collaborator_count")]
    pub collaborator_count: u64,
    pub affiliation: ::serde_json::Value,
    #[serde(rename = "hub_user")]
    pub hub_user: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub creator: i64,
    pub id: i64,
    #[serde(rename = "image_id")]
    pub image_id: ::serde_json::Value,
    pub images: Vec<Image>,
    #[serde(rename = "last_updated")]
    pub last_updated: Option<String>,
    #[serde(rename = "last_updater")]
    pub last_updater: i64,
    #[serde(rename = "last_updater_username")]
    pub last_updater_username: String,
    pub name: String,
    pub repository: i64,
    #[serde(rename = "full_size")]
    pub full_size: i64,
    pub v2: bool,
    #[serde(rename = "tag_status")]
    pub tag_status: String,
    #[serde(rename = "tag_last_pulled")]
    pub tag_last_pulled: Option<String>,
    #[serde(rename = "tag_last_pushed")]
    pub tag_last_pushed: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub architecture: String,
    pub features: String,
    pub variant: Option<String>,
    pub digest: Option<String>,
    pub os: String,
    #[serde(rename = "os_features")]
    pub os_features: String,
    #[serde(rename = "os_version")]
    pub os_version: ::serde_json::Value,
    pub size: i64,
    pub status: String,
    #[serde(rename = "last_pulled")]
    pub last_pulled: Option<String>,
    #[serde(rename = "last_pushed")]
    pub last_pushed: Option<String>,
}
