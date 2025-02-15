use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Match {
    pub match_details: MatchDetails,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct MatchDetails {
    pub match_uid: String,

    pub game_mode: GameMode,

    pub replay_id: String,

    pub mvp_uid: i64,

    pub mvp_hero_id: i64,

    pub svp_uid: i64,

    pub svp_hero_id: i64,

    pub dynamic_fields: DynamicFields,

    pub match_players: Vec<MatchPlayer>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynamicFields {
    pub ban_pick_info: Option<Vec<BanPickInfo>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BanPickInfo {
    pub battle_side: i64,

    pub hero_id: i64,

    pub is_pick: i64,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameMode {
    pub game_mode_id: i64,

    pub game_mode_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct MatchPlayer {
    pub player_uid: i64,

    pub nick_name: String,

    pub player_icon: i64,

    pub camp: i64,

    pub cur_hero_id: i64,

    pub cur_hero_icon: String,

    pub is_win: i64,

    pub kills: i64,

    pub deaths: i64,

    pub assists: i64,

    pub total_hero_damage: f64,

    pub total_hero_heal: f64,

    pub total_damage_taken: f64,

    pub player_heroes: Vec<PlayerHero>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct PlayerHero {
    pub hero_id: i64,

    pub play_time: f64,

    pub kills: i64,

    pub deaths: i64,

    pub assists: i64,

    pub session_hit_rate: f64,

    pub hero_icon: String,
}
