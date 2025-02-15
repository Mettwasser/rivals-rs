use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemType {
    Emote,
    Mvp,
    Nameplate,
    Spray,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Items {
    pub total_items: i64,

    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub id: String,

    pub name: String,

    pub quality: String,

    #[serde(rename = "type")]
    pub item_type: ItemType,

    pub associated_hero: String,

    pub icon: String,

    pub slug: String,

    pub description: Option<String>,
}
