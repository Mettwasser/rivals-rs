use rivals::Match;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn match_() -> &'static str {
    include_str!("data/match.json")
}

#[rstest]
fn test_match_(match_: &str) {
    let _match_: Match = serde_json::from_str(match_).unwrap();
}
