use rivals::Skin;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn skin() -> &'static str {
    include_str!("data/skin.json")
}

#[rstest]
fn test_skin(skin: &str) {
    let _skin: Skin = serde_json::from_str(skin).unwrap();
}
