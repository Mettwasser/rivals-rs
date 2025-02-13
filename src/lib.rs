pub mod client;
pub mod models;

pub use models::{
    battlepass::*,
    hero::*,
    leaderboard::*,
    Platform,
};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    Reqwest(#[from] reqwest::Error),
}
