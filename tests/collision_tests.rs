// tests/collision_tests.rs

use snake_game::core::CollisionManager;
use snake_game::entities::Point;  // Removed unused Obstacle
use std::collections::VecDeque;

#[test]
fn test_wall_collision() {
    let manager = CollisionManager::new(50, 25);
    
    // Test boundaries
    assert!(manager.is_wall_collision(&Point::new(0, 10)), "Should collide with left wall");
    assert!(manager.is_wall_collision(&Point::new(49, 10)), "Should collide with right wall");
    assert!(manager.is_wall_collision(&Point::new(10, 0)), "Should collide with top wall");
    assert!(manager.is_wall_collision(&Point::new(10, 24)), "Should collide with bottom wall");
    
    // Test safe position
    assert!(!manager.is_wall_collision(&Point::new(10, 10)), "Should not collide in middle");
}

#[test]
fn test_self_collision() {
    let manager = CollisionManager::new(50, 25);
    let mut snake = VecDeque::new();
    
    // Create a snake that intersects with itself
    snake.push_back(Point::new(5, 5));
    snake.push_back(Point::new(6, 5));
    snake.push_back(Point::new(6, 6));
    snake.push_back(Point::new(5, 6));
    snake.push_back(Point::new(5, 5)); // Collides with first point
    
    assert!(manager.is_self_collision(&snake), "Should detect self collision");
    
    // Test non-colliding snake
    let mut straight_snake = VecDeque::new();
    straight_snake.push_back(Point::new(5, 5));
    straight_snake.push_back(Point::new(6, 5));
    straight_snake.push_back(Point::new(7, 5));
    
    assert!(!manager.is_self_collision(&straight_snake), "Should not detect collision in straight snake");
}