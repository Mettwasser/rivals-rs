use serde::Deserialize;
use serde_this_or_that::as_f64;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Leaderboard {
    #[serde(rename = "_id")]
    pub id: i64,

    pub players: Vec<Player>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Player {
    info: Info,

    pub player_uid: i64,

    pub matches: i64,

    pub wins: i64,

    pub kills: i64,

    pub deaths: i64,

    pub assists: i64,

    #[serde(deserialize_with = "as_f64")]
    pub play_time: f64,

    #[serde(deserialize_with = "as_f64")]
    pub total_hero_damage: f64,

    #[serde(deserialize_with = "as_f64")]
    pub total_damage_taken: f64,

    #[serde(deserialize_with = "as_f64")]
    pub total_hero_heal: f64,

    pub mvps: i64,

    pub svps: i64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Info {
    name: String,

    pub cur_head_icon_id: String,

    pub rank_season: RankSeason,

    pub login_os: String,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RankSeason {
    rank_game_id: i64,

    pub level: i64,

    #[serde(deserialize_with = "as_f64")]
    pub rank_score: f64,

    pub max_level: i64,

    #[serde(deserialize_with = "as_f64")]
    pub max_rank_score: f64,

    pub update_time: i64,

    pub win_count: i64,

    pub protect_score: i64,

    #[serde(deserialize_with = "as_f64")]
    pub diff_score: f64,
}
