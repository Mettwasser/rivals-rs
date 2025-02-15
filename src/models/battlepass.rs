use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BattlePass {
    pub season: i64,

    pub season_name: String,

    pub items: Vec<BattlepassItem>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct BattlepassItem {
    pub name: String,

    pub image: String,

    pub cost: String,

    /// Whether it's part of the free or premium track
    pub is_luxury: bool,
}
