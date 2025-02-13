use rivals::Hero;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn heroes() -> &'static str {
    include_str!("data/heroes.json")
}

#[rstest]
fn test_heroes(heroes: &str) {
    let _heroes: Vec<Hero> = serde_json::from_str(heroes).unwrap();
}
