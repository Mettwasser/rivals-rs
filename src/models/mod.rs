use derive_more::Display;
use serde::Deserialize;

pub mod battlepass;
pub mod hero;
pub mod leaderboard;

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
