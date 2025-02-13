use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BattlePass {
    season: i64,

    season_name: String,

    items: Vec<Item>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    name: String,

    image: String,

    cost: String,

    /// Whether it's part of the free or premium track
    is_luxury: bool,
}
