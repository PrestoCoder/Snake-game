// tests/food_tests.rs
use snake_game::entities::Food;
use snake_game::entities::Point;

#[test]
fn test_food_generation() {
    let width = 50;
    let height = 25;
    
    let food = Food::generate_new(
        width,
        height,
        |point| point.x > 5 && point.y > 5  // Test position validator
    );
    
    let pos = food.position();
    assert!(pos.x > 5 && pos.y > 5, "Food should respect position validator");
    assert!(pos.x < width && pos.y < height, "Food should be within bounds");
}

#[test]
fn test_food_position() {
    let test_point = Point::new(10, 10);
    let food = Food::new(test_point);
    assert_eq!(*food.position(), test_point, "Food should maintain its position");
}