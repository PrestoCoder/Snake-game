// tests/scoring_tests.rs
use snake_game::core::GameState;
use snake_game::entities::{Direction, Point};
use snake_game::gameplay::GameState as GameStateEnum;
use snake_game::config::*;  // Import constants

fn is_move_safe(game: &GameState, next_pos: &Point) -> bool {
    // Use WIDTH and HEIGHT from config
    if next_pos.x >= WIDTH || next_pos.y >= HEIGHT {
        println!("Move to ({}, {}) would be out of bounds", next_pos.x, next_pos.y);
        return false;
    }

    // Check if position would hit snake body
    if game.snake().body().contains(next_pos) {
        println!("Move to ({}, {}) would hit snake body", next_pos.x, next_pos.y);
        return false;
    }

    // Check if position would hit obstacle
    if game.obstacles().iter().any(|obstacle| obstacle.blocks.contains(next_pos)) {
        println!("Move to ({}, {}) would hit obstacle", next_pos.x, next_pos.y);
        return false;
    }

    true
}

fn move_towards_food(game: &mut GameState) -> bool {
    if let Some(head) = game.snake().head() {
        let food_pos = game.food().position();
        let dx = food_pos.x as i32 - head.x as i32;
        let dy = food_pos.y as i32 - head.y as i32;
        
        println!("\nCurrent state:");
        println!("Head: ({}, {})", head.x, head.y);
        println!("Food: ({}, {})", food_pos.x, food_pos.y);
        println!("Distance: dx={}, dy={}", dx, dy);
        println!("Current direction: {:?}", game.snake().direction());

        // Calculate next positions for each direction
        let next_positions = [
            (Direction::Up, Point::new(head.x, head.y.wrapping_sub(1))),
            (Direction::Down, Point::new(head.x, head.y.wrapping_add(1))),
            (Direction::Left, Point::new(head.x.wrapping_sub(1), head.y)),
            (Direction::Right, Point::new(head.x.wrapping_add(1), head.y)),
        ];

        // Find all safe moves
        let mut safe_moves: Vec<_> = next_positions.into_iter()
            .filter(|(_, pos)| is_move_safe(game, pos))
            .collect();

        if safe_moves.is_empty() {
            println!("No safe moves available!");
            return false;
        }

        // Sort moves by how much closer they get us to the food
        safe_moves.sort_by_key(|(_, pos)| {
            let new_dx = (food_pos.x as i32 - pos.x as i32).abs();
            let new_dy = (food_pos.y as i32 - pos.y as i32).abs();
            new_dx + new_dy  // Manhattan distance
        });

        // Take the move that gets us closest to the food
        let (best_dir, _) = safe_moves[0];
        println!("Choosing direction: {:?}", best_dir);
        game.change_direction(best_dir);
        true
    } else {
        println!("No snake head found!");
        false
    }
}

#[test]
fn test_score_increment() {
    let mut game = GameState::new();
    let initial_score = game.score();
    let mut moves = 0;
    let max_moves = 200;
    let mut last_position = None;
    let mut stuck_count = 0;
    
    println!("\nStarting score increment test");
    println!("Initial state:");
    println!("Snake length: {}", game.snake().body().len());
    println!("Initial score: {}", initial_score);
    println!("Number of obstacles: {}", game.obstacles().len());
    
    while moves < max_moves {
        // Check if we're stuck
        if let Some(head) = game.snake().head() {
            if let Some(last_pos) = last_position {
                if last_pos == *head {
                    stuck_count += 1;
                    if stuck_count > 5 {
                        println!("Snake appears to be stuck!");
                        break;
                    }
                } else {
                    stuck_count = 0;
                }
            }
            last_position = Some(*head);
        }

        if !move_towards_food(&mut game) {
            println!("No valid moves available - stopping");
            break;
        }
        
        game.update().unwrap();
        moves += 1;
        
        if game.score() > initial_score {
            println!("\nSuccess! Food collected at move {}", moves);
            break;
        }
        
        if matches!(game.game_state(), GameStateEnum::GameOver(_)) {
            println!("\nGame over at move {}", moves);
            if let Some(head) = game.snake().head() {
                println!("Final position: ({}, {})", head.x, head.y);
                println!("Snake body positions:");
                for (i, pos) in game.snake().body().iter().enumerate() {
                    println!("  Segment {}: ({}, {})", i, pos.x, pos.y);
                }
            }
            break;
        }
    }
    
    assert!(game.score() > initial_score, 
        "Score should increase after collecting food. Moves: {}, Final score: {}", 
        moves, game.score());
}


// ... rest of tests remain the same ...

#[test]
fn test_speed_increase() {
    let mut game = GameState::new();
    let initial_tick_rate = game.get_tick_rate();
    let mut moves = 0;
    let max_moves = 200;
    
    println!("Testing speed increase");
    println!("Initial tick rate: {}", initial_tick_rate);
    
    while moves < max_moves {
        if let Some(head) = game.snake().head() {
            println!("Move {}: Snake at ({}, {}), Food at ({}, {}), Speed: {}", 
                    moves,
                    head.x, head.y,
                    game.food().position().x, game.food().position().y,
                    game.speed_level());
        }
        
        move_towards_food(&mut game);
        game.update().unwrap();
        moves += 1;
        
        if game.speed_level() > 1 {
            println!("Speed increased at move {}", moves);
            break;
        }
        
        if matches!(game.game_state(), snake_game::gameplay::GameState::GameOver(_)) {
            println!("Game over at move {}", moves);
            break;
        }
    }
    
    assert!(game.get_tick_rate() < initial_tick_rate, 
        "Tick rate should decrease (speed up) after collecting food. Moves: {}", moves);
}

#[test]
fn test_score_persistence() {
    let mut game = GameState::new();
    let initial_score = game.score();
    let mut collected_food = false;
    
    // First collect some food
    for moves in 0..200 {
        move_towards_food(&mut game);
        game.update().unwrap();
        
        if game.score() > initial_score {
            collected_food = true;
            println!("Collected food at move {}", moves);
            break;
        }
    }
    
    assert!(collected_food, "Should be able to collect food");
    let score_after_food = game.score();
    
    // Score should persist
    for _ in 0..10 {
        game.update().unwrap();
    }
    
    assert_eq!(game.score(), score_after_food, "Score should persist between updates");
}