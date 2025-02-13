use derive_builder::Builder;
use reqwest::RequestBuilder;
use url::Url;

use crate::{
    BattlePass,
    Error,
    Heroes,
    Leaderboard,
    Platform,
};

#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned")]
pub struct Client {
    #[builder(default)]
    pub http: reqwest::Client,

    #[builder(default = "https://marvelrivalsapi.com/api/v1".parse().unwrap())]
    pub base_url: Url,

    pub api_key: String,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn request(&self, url: &str) -> RequestBuilder {
        self.http.get(url).header("X-Api-Key", &self.api_key)
    }

    pub async fn get_heroes(&self) -> Result<Heroes, Error> {
        let url = format!("{}/heroes", self.base_url);

        let heroes = self.request(&url).send().await?.json().await?;

        Ok(heroes)
    }

    pub async fn get_battlepass(&self, season: u32) -> Result<BattlePass, Error> {
        let url = format!("{}/battlepass", self.base_url);

        let battlepass = self
            .request(&url)
            .query(&[("season", &season.to_string())])
            .send()
            .await?
            .json()
            .await?;

        Ok(battlepass)
    }

    /// Please note that this endpoint is huge and may take a while to return.
    pub async fn get_leaderboard(
        &self,
        hero: &str,
        platform: Platform,
    ) -> Result<Leaderboard, Error> {
        let url = format!("{}/heroes/leaderboard/{hero}", self.base_url);

        let leaderboard = self
            .request(&url)
            .query(&[("platform", &platform.to_string())])
            .send()
            .await?
            .json()
            .await?;

        Ok(leaderboard)
    }
}

#[cfg(test)]
#[tokio::test]
async fn main() {
    dotenv::dotenv().ok();
    let client = Client::builder()
        .api_key(std::env::var("API_KEY").unwrap())
        .build()
        .unwrap();

    let _leaderboard = client.get_leaderboard("hulk", Platform::Pc).await.unwrap();

    // println!("{:?}", _leaderboard);
}
