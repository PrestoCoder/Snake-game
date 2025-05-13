// tests/tick_rate_tests.rs
use snake_game::core::GameState;

#[test]
fn test_initial_tick_rate() {
    let game = GameState::new();
    let initial_rate = game.get_tick_rate();
    assert!(initial_rate > 0, "Tick rate should be positive");
    assert!(initial_rate <= 200, "Initial tick rate should not exceed 200ms");
}

#[test]
fn test_tick_rate_bounds() {
    let game = GameState::new();
    let min_rate = game.get_tick_rate();
    assert!(min_rate >= 50, "Tick rate should not go below 50ms");
}