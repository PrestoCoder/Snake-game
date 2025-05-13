// tests/snake_tests.rs

use snake_game::gameplay::Snake;
use snake_game::entities::{Point, Direction};

#[test]
fn test_snake_initialization() {
    let snake = Snake::new(10, 10);
    assert_eq!(snake.body().len(), 3, "Snake should start with length 3");
    
    let body_vec: Vec<_> = snake.body().iter().collect();
    assert_eq!(*body_vec[2], Point::new(10, 10), "Snake head should be at specified position");
}

#[test]
fn test_snake_movement() {
    let mut snake = Snake::new(10, 10);
    let _initial_head = *snake.head().unwrap();  // Added underscore to acknowledge unused variable
    // or we could remove this line since it's not being used
    
    snake.move_forward(Point::new(11, 10));
    assert_eq!(snake.body().len(), 4, "Snake length should increase after moving");
    assert_eq!(*snake.head().unwrap(), Point::new(11, 10), "Snake head should move to new position");
    
    snake.retract_tail();
    assert_eq!(snake.body().len(), 3, "Snake length should decrease after retracting");
}

#[test]
fn test_snake_direction_change() {
    let mut snake = Snake::new(10, 10);
    
    // Test valid direction changes
    snake.change_direction(Direction::Up);
    assert_eq!(snake.direction(), Direction::Up);
    
    snake.change_direction(Direction::Right);
    assert_eq!(snake.direction(), Direction::Right);
    
    // Test invalid direction change (opposite direction)
    snake.change_direction(Direction::Left);
    assert_eq!(snake.direction(), Direction::Right, "Snake shouldn't be able to reverse direction");
}