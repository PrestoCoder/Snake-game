// tests/integration_tests.rs
use snake_game::core::GameState;

#[test]
fn test_game_progression() {
    let game = GameState::new();  // Removed mut as it's not needed
    
    // Test initial state
    assert_eq!(game.current_level(), 1, "Game should start at level 1");
    assert_eq!(game.score(), 0, "Initial score should be 0");
    
    let food_pos = *game.food().position();
    assert!(food_pos.x > 0 && food_pos.x < 50, "Food x position should be within bounds");
    assert!(food_pos.y > 0 && food_pos.y < 25, "Food y position should be within bounds");
}

#[test]
fn test_game_over_conditions() {
    let game = GameState::new();  // Removed mut
    assert!(matches!(game.game_state(), snake_game::gameplay::GameState::Playing));
}

#[test]
fn test_score_and_speed() {
    let game = GameState::new();
    
    assert_eq!(game.score(), 0, "Game should start with score 0");
    assert_eq!(game.speed_level(), 1, "Game should start at speed level 1");
}