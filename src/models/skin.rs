use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Skin {
    pub id: String,

    pub name: String,

    pub icon: String,

    pub rarity: String,

    pub description: String,

    pub appearance: String,
}
