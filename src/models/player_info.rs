use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerInfo {
    name: String,

    uid: String,
}
