use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct HeroStats {
    pub hero_id: i64,

    pub hero_name: String,

    pub hero_icon: String,

    pub matches: i64,

    pub wins: i64,

    pub k: f64,

    pub d: f64,

    pub a: f64,

    pub play_time: String,

    pub total_hero_damage: f64,

    pub total_hero_heal: i64,

    pub total_damage_taken: f64,

    pub session_hit_rate: f64,

    pub solo_kill: f64,
}
