// tests/movement_tests.rs
use snake_game::core::GameState;
use snake_game::entities::Direction;

#[test]
fn test_basic_movement() {
    let mut game = GameState::new();
    let initial_head = *game.snake().head().unwrap();
    game.update().unwrap();
    let new_head = *game.snake().head().unwrap();
    assert!(new_head.x > initial_head.x, "Snake should move right initially");
}

#[test]
fn test_direction_change() {
    let mut game = GameState::new();
    game.change_direction(Direction::Up);
    assert_eq!(game.snake().direction(), Direction::Up);
    
    // Test opposite direction
    game.change_direction(Direction::Down);
    assert_eq!(game.snake().direction(), Direction::Up, 
        "Should not be able to move in opposite direction");
}

#[test]
fn test_wall_collision() {
    let mut game = GameState::new();
    // Move right until hitting wall
    for _ in 0..100 {
        game.update().unwrap();
        if matches!(game.game_state(), snake_game::gameplay::GameState::GameOver(_)) {
            break;
        }
    }
    assert!(matches!(game.game_state(), snake_game::gameplay::GameState::GameOver(_)));
}