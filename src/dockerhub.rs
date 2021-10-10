//!
use reqwest::Result;

const PER_PAGE: u64 = 100;

pub struct Repositories {
    owner: String,
    repositories: <Vec<Repository> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u64,
    per_page: u64,
    total: u64,
}

impl Repositories {
    pub fn of(owner: &str) -> Result<Self> {
        Ok(Repositories {
            owner: owner.to_owned(),
            repositories: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: PER_PAGE,
            total: 0,
        })
    }

    fn try_next(&mut self) -> Result<Option<Repository>> {
        if let Some(repo) = self.repositories.next() {
            return Ok(Some(repo));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!(
            "https://hub.docker.com/v2/repositories/{}/?page={}&page_size={}",
            self.owner, self.page, self.per_page
        );

        println!("url: {}", url);

        let response = self
            .client
            .get(&url)
            .send()?
            .json::<RepositoriesResponse>()?;
        self.repositories = response.results.into_iter();
        self.total = response.count;
        Ok(self.repositories.next())
    }
}

impl Iterator for Repositories {
    type Item = Result<Repository>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(repo)) => Some(Ok(repo)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

// Owner repositories
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoriesResponse {
    pub count: u64,
    pub next: ::serde_json::Value,
    pub previous: ::serde_json::Value,
    pub results: Vec<Repository>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
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

// Repository tags
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsResponse {
    pub count: i64,
    pub next: String,
    pub previous: ::serde_json::Value,
    pub results: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub creator: i64,
    pub id: i64,
    #[serde(rename = "image_id")]
    pub image_id: ::serde_json::Value,
    pub images: Vec<Image>,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
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
    pub tag_last_pulled: String,
    #[serde(rename = "tag_last_pushed")]
    pub tag_last_pushed: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub architecture: String,
    pub features: String,
    pub variant: ::serde_json::Value,
    pub digest: String,
    pub os: String,
    #[serde(rename = "os_features")]
    pub os_features: String,
    #[serde(rename = "os_version")]
    pub os_version: ::serde_json::Value,
    pub size: i64,
    pub status: String,
    #[serde(rename = "last_pulled")]
    pub last_pulled: String,
    #[serde(rename = "last_pushed")]
    pub last_pushed: String,
}
