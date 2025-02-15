use rivals::Items;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn items() -> &'static str {
    include_str!("data/items.json")
}

#[rstest]
fn test_items(items: &str) {
    let _items: Items = serde_json::from_str(items).unwrap();
}
