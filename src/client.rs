use derive_builder::Builder;
use reqwest::{
    RequestBuilder,
    StatusCode,
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    BattlePass,
    Error,
    Hero,
    HeroStats,
    Item,
    Items,
    Leaderboard,
    Match,
    Platform,
    PlayerInfo,
    Skin,
};

async fn fetch<T>(req: RequestBuilder) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let resp = req.send().await?;
    let status = resp.status();

    match status.is_success() {
        true => Ok(resp.json().await?),
        false => Err(Error::ApiError(resp.json().await?)),
    }
}

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

    fn new_request<S>(&self, url: S) -> RequestBuilder
    where
        S: AsRef<str>,
    {
        self.http
            .get(format!("{}{}", self.base_url, url.as_ref()))
            .header("X-Api-Key", &self.api_key)
    }

    // region: BattlePass

    /// Get battlepass data for a specific season
    pub async fn get_battlepass(&self, season: u32) -> Result<BattlePass, Error> {
        let req = self
            .new_request("/battlepass")
            .query(&[("season", &season.to_string())]);

        fetch(req).await
    }
    // endregion

    // region: Heroes

    /// Get all currently available heroes
    pub async fn get_heroes(&self) -> Result<Vec<Hero>, Error> {
        let req = self.new_request("/heroes");

        fetch(req).await
    }

    /// Search hero by their ID or name
    pub async fn get_hero(&self, query: &str) -> Result<Hero, Error> {
        let req = self.new_request(format!("/heroes/hero/{query}"));

        fetch(req).await
    }

    /// Get hero statistics by their ID or name
    pub async fn get_hero_stats(&self, query: &str) -> Result<HeroStats, Error> {
        let req = self.new_request(format!("/heroes/hero/{query}/stats"));

        fetch(req).await
    }

    /// Get all skins (or "costumes") for a specific hero
    pub async fn get_hero_skins(&self, hero: &str) -> Result<Vec<Skin>, Error> {
        let req = self.new_request(format!("/heroes/hero/{hero}/costumes"));

        fetch(req).await
    }

    /// Get a specific skin for a hero
    pub async fn get_hero_skin(&self, hero: &str, skin: &str) -> Result<Skin, Error> {
        let req = self.new_request(format!("/heroes/hero/{hero}/costume/{skin}"));

        fetch(req).await
    }

    /// Get the leaderboard for a specific hero by their ID or name
    /// Please note that this endpoint is huge and may take a while to return
    pub async fn get_hero_leaderboard(
        &self,
        hero: &str,
        platform: Platform,
    ) -> Result<Leaderboard, Error> {
        let req = self
            .new_request(format!("/heroes/leaderboard/{hero}"))
            .query(&[("platform", &platform.to_string())]);

        fetch(req).await
    }
    // endregion

    // region: Items

    /// Get paginated items
    /// Please note that this endpoint is huge and may take a while to return
    pub async fn get_items(&self, page: u32, limit: u32) -> Result<Items, Error> {
        let req = self
            .new_request("/items")
            .query(&[("page", &page.to_string()), ("limit", &limit.to_string())]);

        fetch(req).await
    }

    /// Search item by its ID or name
    pub async fn get_item(&self, query: &str) -> Result<Item, Error> {
        let req = self.new_request(format!("/item/{query}"));

        fetch(req).await
    }

    // endregion

    // region: Match

    pub async fn get_match(&self, match_uid: &str) -> Result<Match, Error> {
        let req = self.new_request(format!("/match/{match_uid}"));

        fetch(req).await
    }

    // endregion

    // region: Player

    /// Get a player's match history by their UID or name
    /// Please note that this endpoint is huge and may take a while to return
    pub async fn get_player_match_history(
        &self,
        player: &str,
        season: u32,
        skip: u32,
        game_mode: u32,
    ) -> Result<Vec<Match>, Error> {
        let req = self
            .new_request(format!("/player/{player}/match-history"))
            .query(&[
                ("season", &season.to_string()),
                ("skip", &skip.to_string()),
                ("game_mode", &game_mode.to_string()),
            ]);

        fetch(req).await
    }

    /// Fetches a player by name or UID
    pub async fn get_player(&self, player: &str) -> Result<PlayerInfo, Error> {
        let req = self.new_request(format!("/find-player/{player}"));

        fetch(req).await
    }

    /// Triggers an update for the API's player data
    pub async fn update_player_data(&self, player: &str) -> Result<StatusCode, Error> {
        let req = self.new_request(format!("/player/{player}/update"));

        Ok(req.send().await?.status())
    }

    // endregion
}
