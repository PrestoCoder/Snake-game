use crate::{
    error::{GameError, Result},
    types::{Direction, Point},
};
use log::debug;
use std::collections::VecDeque;

const BORDER_THICKNESS: u16 = 2;

pub struct GameState {
    snake: VecDeque<Point>,
    food: Point,
    direction: Direction,
    score: u32,
    dimensions: (u16, u16),
    game_over: bool,
}

impl GameState {
    pub fn new(width: u16, height: u16) -> Self {
        let mut snake = VecDeque::new();
        let center_x = width / 2;
        let center_y = height / 2;
        
        // Initialize snake with 3 segments
        snake.push_back(Point::new(center_x - 2, center_y));
        snake.push_back(Point::new(center_x - 1, center_y));
        snake.push_back(Point::new(center_x, center_y));

        let food = Self::generate_food_avoiding_snake(width, height, &snake);

        Self {
            snake,
            food,
            direction: Direction::Right,
            score: 0,
            dimensions: (width, height),
            game_over: false,
        }
    }

    pub fn update(&mut self) -> Result<()> {
        if self.game_over {
            return Ok(());
        }

        // Get current head position
        let current_head = self.snake.back()
            .ok_or_else(|| GameError::GameState("Snake has no head".to_string()))?;
        
        // Calculate new head position
        let new_head = current_head.translate(&self.direction);

        // Debug print current state
        debug!("Current head: ({}, {}), New head: ({}, {}), Direction: {:?}", 
            current_head.x, current_head.y, new_head.x, new_head.y, self.direction);

        // Check wall collision
        if self.is_wall_collision(&new_head) {
            debug!("Wall collision at ({}, {}). Bounds: ({}, {})", 
                new_head.x, new_head.y, self.dimensions.0, self.dimensions.1);
            self.game_over = true;
            return Ok(());
        }

        // Check self collision
        let will_remove_tail = new_head != self.food;
        let start_idx = if will_remove_tail { 1 } else { 0 };
        
        // Convert snake to Vec for easier iteration
        let snake_vec: Vec<_> = self.snake.iter().collect();
        for i in start_idx..snake_vec.len() {
            if &new_head == snake_vec[i] {
                debug!("Self collision at ({}, {})", new_head.x, new_head.y);
                self.game_over = true;
                return Ok(());
            }
        }

        // Move snake
        self.snake.push_back(new_head);

        // Check if food was eaten
        if new_head == self.food {
            debug!("Food eaten at ({}, {})", self.food.x, self.food.y);
            self.score += 1;
            // Generate new food and ensure it's not on the snake
            self.food = Self::generate_food_avoiding_snake(
                self.dimensions.0,
                self.dimensions.1,
                &self.snake
            );
            debug!("New food generated at ({}, {})", self.food.x, self.food.y);
        } else {
            // Remove tail only if food wasn't eaten
            self.snake.pop_front();
        }

        debug!("Snake length: {}, Score: {}", self.snake.len(), self.score);
        Ok(())
    }

    pub fn is_wall_collision(&self, point: &Point) -> bool {
        point.x < BORDER_THICKNESS || 
        point.x >= self.dimensions.0 - BORDER_THICKNESS || 
        point.y < BORDER_THICKNESS || 
        point.y >= self.dimensions.1 - BORDER_THICKNESS
    }

    fn generate_food_avoiding_snake(width: u16, height: u16, snake: &VecDeque<Point>) -> Point {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        loop {
            let food = Point::new(
                rng.gen_range(BORDER_THICKNESS..width - BORDER_THICKNESS),
                rng.gen_range(BORDER_THICKNESS..height - BORDER_THICKNESS),
            );
            
            // Make sure food doesn't appear on snake
            if !snake.contains(&food) {
                return food;
            }
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // Get current head and neck positions
        if let (Some(head), Some(neck)) = (self.snake.back(), self.snake.get(self.snake.len() - 2)) {
            // Calculate where the head would be after moving in the new direction
            let potential_next = head.translate(&new_direction);
            
            // Only change direction if it wouldn't cause immediate collision with neck
            if potential_next != *neck {
                debug!("Direction changed from {:?} to {:?}", self.direction, new_direction);
                self.direction = new_direction;
            }
        }
    }

    // Getters
    pub fn snake(&self) -> &VecDeque<Point> { &self.snake }
    pub fn food(&self) -> &Point { &self.food }
    pub fn score(&self) -> u32 { self.score }
    pub fn is_game_over(&self) -> bool { self.game_over }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let game = GameState::new(20, 20);
        assert_eq!(game.snake().len(), 3);
        assert!(!game.is_game_over());
    }

    #[test]
    fn test_wall_collision() {
        let mut game = GameState::new(10, 10);
        // Move snake to wall
        while !game.is_game_over() {
            game.update().unwrap();
        }
        assert!(game.is_game_over());
    }

    #[test]
    fn test_food_collection() {
        let mut game = GameState::new(20, 20);
        let initial_length = game.snake().len();
        
        // Place food right in front of snake
        let head = *game.snake().back().unwrap();
        game.food = head.translate(&game.direction);
        
        game.update().unwrap();
        
        assert_eq!(game.snake().len(), initial_length + 1);
        assert_eq!(game.score(), 1);
    }
}