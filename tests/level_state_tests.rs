// tests/level_state_tests.rs

use snake_game::gameplay::LevelState;

#[test]
fn test_level_advancement() {
    let mut state = LevelState::new(1, 3, 5);
    
    assert!(!state.should_advance(4), "Should not advance at score 4");
    assert!(state.should_advance(5), "Should advance at score 5");
    
    state.advance();
    assert_eq!(state.current_level, 2, "Should be at level 2");
    
    assert!(!state.should_advance(9), "Should not advance at score 9");
    assert!(state.should_advance(10), "Should advance at score 10");
}

#[test]
fn test_score_needed_calculation() {
    let state = LevelState::new(1, 3, 5);
    assert_eq!(state.score_needed_for_next(), Some(5), "Level 1 should need 5 points");
    
    let final_level = LevelState::new(3, 3, 5);  // Removed unnecessary mut
    assert_eq!(final_level.score_needed_for_next(), None, "Final level should not have next score");
}