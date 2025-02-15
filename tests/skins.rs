use rivals::Skin;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn skins() -> &'static str {
    include_str!("data/skins.json")
}

#[rstest]
fn test_skins(skins: &str) {
    let _skins: Vec<Skin> = serde_json::from_str(skins).unwrap();
}
