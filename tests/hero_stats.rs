use rivals::HeroStats;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn hero_stats() -> &'static str {
    include_str!("data/hero_stats.json")
}

#[rstest]
fn test_hero_stats(hero_stats: &str) {
    let _hero_stats: HeroStats = serde_json::from_str(hero_stats).unwrap();
}
