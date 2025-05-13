// tests/initial_state_tests.rs
use snake_game::core::GameState;
use snake_game::gameplay::GameState as GameStateEnum;
use snake_game::entities::Direction;

#[test]
fn test_initial_state() {
    let game = GameState::new();
    assert!(matches!(game.game_state(), GameStateEnum::Playing));
    assert_eq!(game.score(), 0);
    assert_eq!(game.current_level(), 1);
    assert_eq!(game.speed_level(), 1);
}

#[test]
fn test_initial_snake() {
    let game = GameState::new();
    assert!(game.snake().body().len() >= 3, "Snake should start with at least 3 segments");
    assert_eq!(game.snake().direction(), Direction::Right);
}