use rivals::BattlePass;
use rstest::{
    fixture,
    rstest,
};

#[fixture]
fn battlepass() -> &'static str {
    include_str!("data/battlepass.json")
}

#[rstest]
fn test_battlepass(battlepass: &str) {
    let _battlepass: BattlePass = serde_json::from_str(battlepass).unwrap();
}
