use rivals::MatchHistory;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn match_history() -> &'static str {
    include_str!("data/match_history.json")
}

#[rstest]
fn test_match_history(match_history: &str) {
    let _match_history: MatchHistory = serde_json::from_str(match_history).unwrap();
}
