use rivals::Leaderboard;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn leaderboard() -> &'static str {
    include_str!("data/leaderboard.json")
}

#[rstest]
fn test_leaderboard(leaderboard: &str) {
    let _leaderboard: Leaderboard = serde_json::from_str(leaderboard).unwrap();
}
