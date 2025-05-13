// tests/obstacle_tests.rs
use snake_game::core::GameState;
use snake_game::config::*;

#[test]
fn test_obstacle_generation() {
    let game = GameState::new();
    let obstacles = game.obstacles();
    assert!(!obstacles.is_empty(), "Game should have obstacles");
}

#[test]
fn test_obstacle_bounds() {
    let game = GameState::new();
    for obstacle in game.obstacles() {
        for point in &obstacle.blocks {
            assert!(point.x < WIDTH, "Obstacle x position should be within bounds");
            assert!(point.y < HEIGHT, "Obstacle y position should be within bounds");
        }
    }
}

#[test]
fn test_obstacle_count() {
    let game = GameState::new();
    let expected_count = (game.current_level() + 1).pow(2) as usize;
    assert_eq!(game.obstacles().len(), expected_count,
        "Should have correct number of obstacles for level");
}