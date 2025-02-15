use derive_more::Display;
use serde::Deserialize;

pub mod battlepass;
pub mod hero;
pub mod hero_stats;
pub mod item;
pub mod leaderboard;
pub mod match_;
pub mod match_history;
pub mod player_info;
pub mod skin;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    #[display("pc")]
    Pc,
    #[display("ps")]
    Ps,
    #[display("xbox")]
    Xbox,
}
