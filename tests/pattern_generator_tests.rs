// tests/pattern_generator_tests.rs

use snake_game::gameplay::get_level_pattern;

#[test]
fn test_obstacle_count_matches_level() {
    let width = 50;
    let height = 25;
    
    // Test for first 3 levels
    for level in 1..=3 {
        let pattern = get_level_pattern(level, width, height);
        let expected_count = (level + 1).pow(2);
        assert_eq!(pattern.positions.len(), expected_count as usize, 
            "Level {} should have {} obstacles", level, expected_count);
        assert_eq!(pattern.sizes.len(), expected_count as usize,
            "Level {} should have {} size entries", level, expected_count);
    }
}

#[test]
fn test_obstacles_within_bounds() {
    let width = 50;
    let height = 25;
    let pattern = get_level_pattern(1, width, height);
    
    for ((x, y), (w, h)) in pattern.positions.iter().zip(pattern.sizes.iter()) {
        assert!(*x + w <= width, "Obstacle extends beyond right boundary");
        assert!(*y + h <= height, "Obstacle extends beyond bottom boundary");
        assert!(*x >= 4, "Obstacle too close to left boundary");
        assert!(*y >= 4, "Obstacle too close to top boundary");
    }
}