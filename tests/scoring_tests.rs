// tests/scoring_tests.rs
use snake_game::core::GameState;
use snake_game::entities::Direction;

// Helper function to move snake towards food
fn move_towards_food(game: &mut GameState) {
    if let Some(head) = game.snake().head() {
        let food_pos = game.food().position();
        
        // First try horizontal movement
        if food_pos.x != head.x {
            game.change_direction(if food_pos.x > head.x {
                Direction::Right
            } else {
                Direction::Left
            });
        } 
        // Then try vertical movement
        else if food_pos.y != head.y {
            game.change_direction(if food_pos.y > head.y {
                Direction::Down
            } else {
                Direction::Up
            });
        }
    }
}

#[test]
fn test_score_increment() {
    let mut game = GameState::new();
    let initial_score = game.score();
    
    // Try to collect food with timeout
    for i in 0..100 {
        println!("Iteration {}: Snake at {:?}, Food at {:?}, Score: {}", 
                i,
                game.snake().head().unwrap(),
                game.food().position(),
                game.score());
        
        move_towards_food(&mut game);
        game.update().unwrap();
        
        if game.score() > initial_score {
            break;
        }
        
        // Check for game over
        if matches!(game.game_state(), snake_game::gameplay::GameState::GameOver(_)) {
            println!("Game over at iteration {}", i);
            break;
        }
    }
    
    assert!(game.score() > initial_score, "Score should increase after collecting food");
}

#[test]
fn test_speed_increase() {
    let mut game = GameState::new();
    let initial_tick_rate = game.get_tick_rate();
    
    // Try to collect food with timeout
    for i in 0..100 {
        println!("Iteration {}: Snake at {:?}, Food at {:?}, Speed level: {}", 
                i,
                game.snake().head().unwrap(),
                game.food().position(),
                game.speed_level());
        
        move_towards_food(&mut game);
        game.update().unwrap();
        
        if game.speed_level() > 1 {
            break;
        }
        
        // Check for game over
        if matches!(game.game_state(), snake_game::gameplay::GameState::GameOver(_)) {
            println!("Game over at iteration {}", i);
            break;
        }
    }
    
    let final_tick_rate = game.get_tick_rate();
    println!("Initial tick rate: {}, Final tick rate: {}", initial_tick_rate, final_tick_rate);
    
    assert!(final_tick_rate < initial_tick_rate, 
        "Tick rate should decrease (speed up) after collecting food");
}

#[test]
fn test_score_persistence() {
    let mut game = GameState::new();
    
    // Get initial score
    let initial_score = game.score();
    
    // Try to collect food
    for _ in 0..100 {
        move_towards_food(&mut game);
        game.update().unwrap();
        
        if game.score() > initial_score {
            break;
        }
    }
    
    let new_score = game.score();
    assert!(new_score > initial_score, "Score should increase");
    
    // Score should persist after more updates
    for _ in 0..10 {
        game.update().unwrap();
    }
    
    assert_eq!(game.score(), new_score, "Score should persist between updates");
}