use serde::Deserialize;
use serde_this_or_that::as_f64;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Leaderboard {
    #[serde(rename = "_id")]
    id: i64,

    players: Vec<Player>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Player {
    info: Info,

    player_uid: i64,

    matches: i64,

    wins: i64,

    kills: i64,

    deaths: i64,

    assists: i64,

    #[serde(deserialize_with = "as_f64")]
    play_time: f64,

    #[serde(deserialize_with = "as_f64")]
    total_hero_damage: f64,

    #[serde(deserialize_with = "as_f64")]
    total_damage_taken: f64,

    #[serde(deserialize_with = "as_f64")]
    total_hero_heal: f64,

    mvps: i64,

    svps: i64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Info {
    name: String,

    cur_head_icon_id: String,

    rank_season: RankSeason,

    login_os: String,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RankSeason {
    rank_game_id: i64,

    level: i64,

    #[serde(deserialize_with = "as_f64")]
    rank_score: f64,

    max_level: i64,

    #[serde(deserialize_with = "as_f64")]
    max_rank_score: f64,

    update_time: i64,

    win_count: i64,

    protect_score: i64,

    #[serde(deserialize_with = "as_f64")]
    diff_score: f64,
}
