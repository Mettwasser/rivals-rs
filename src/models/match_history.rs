use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MatchHistory {
    pub match_history: Vec<MatchHistoryDetails>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct MatchHistoryDetails {
    pub match_map_id: i64,

    pub map_thumbnail: String,

    pub match_play_duration: f64,

    pub match_season: i64,

    pub match_uid: String,

    pub match_winner_side: i64,

    pub mvp_uid: i64,

    pub svp_uid: i64,

    pub score_info: Option<HashMap<String, i64>>,

    pub match_time_stamp: i64,

    pub play_mode_id: i64,

    pub game_mode_id: i64,

    pub match_player: MatchHistoryPlayer,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct MatchHistoryPlayer {
    pub assists: i64,

    pub kills: i64,

    pub deaths: i64,

    pub is_win: IsWin,

    pub disconnected: bool,

    pub player_uid: i64,

    pub camp: Option<i64>,

    pub score_info: ScoreInfo,

    pub player_hero: MatchHistoryPlayerHero,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct IsWin {
    pub score: i64,

    pub is_win: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct MatchHistoryPlayerHero {
    hero_id: i64,

    pub hero_name: String,

    pub hero_type: String,

    pub kills: i64,

    pub deaths: i64,

    pub assists: i64,

    pub play_time: f64,

    pub total_hero_damage: f64,

    pub total_damage_taken: f64,

    pub total_hero_heal: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct ScoreInfo {
    pub add_score: f64,

    pub level: i64,

    pub new_level: i64,

    pub new_score: f64,
}
