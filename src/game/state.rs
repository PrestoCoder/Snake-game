use crate::{
    error::{GameError, Result},
    types::{Direction, Point},
};
use log::debug;
use std::collections::VecDeque;

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
        // Start snake in the middle, but ensure it's not too close to walls
        let center_x = width / 2;
        let center_y = height / 2;
        
        // Initialize snake with 3 segments, starting more towards the left
        // to give room for initial rightward movement
        snake.push_back(Point::new(center_x - 2, center_y));
        snake.push_back(Point::new(center_x - 1, center_y));
        snake.push_back(Point::new(center_x, center_y));

        // Create initial food away from snake
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

        // Calculate new head position
        let current_head = self.snake.back()
            .ok_or_else(|| GameError::GameState("Snake has no head".to_string()))?;
        let new_head = current_head.translate(&self.direction);

        // Check wall collision
        if self.is_wall_collision(&new_head) {
            debug!("Wall collision detected at {:?}", new_head);
            self.game_over = true;
            return Ok(());
        }

        // Check self collision - don't check with the tail if we're moving
        let mut body_segments: Vec<_> = self.snake.iter().collect();
        body_segments.pop(); // Remove the head
        if body_segments.iter().any(|&segment| segment == &new_head) {
            debug!("Self collision detected at {:?}", new_head);
            self.game_over = true;
            return Ok(());
        }

        // Move snake by adding new head
        self.snake.push_back(new_head);

        // Check if food was eaten
        if new_head == self.food {
            // Increase score and generate new food
            self.score += 1;
            self.food = Self::generate_food_avoiding_snake(
                self.dimensions.0,
                self.dimensions.1,
                &self.snake
            );
            debug!("Food eaten! Score: {}", self.score);
        } else {
            // Remove tail only if food wasn't eaten
            self.snake.pop_front();
        }

        Ok(())
    }

    fn generate_food_avoiding_snake(width: u16, height: u16, snake: &VecDeque<Point>) -> Point {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        loop {
            let food = Point::new(
                rng.gen_range(2..width - 2),  // Keep food away from walls
                rng.gen_range(2..height - 2),
            );
            
            // Make sure food doesn't appear on snake
            if !snake.contains(&food) {
                return food;
            }
        }
    }

    fn is_wall_collision(&self, point: &Point) -> bool {
        point.x <= 0 || point.x >= self.dimensions.0 - 1 
            || point.y <= 0 || point.y >= self.dimensions.1 - 1
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // Prevent 180-degree turns
        if new_direction != self.direction.opposite() {
            self.direction = new_direction;
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
        
        // Check initial snake position is valid
        let head = game.snake().back().unwrap();
        assert!(head.x > 0 && head.x < 19);
        assert!(head.y > 0 && head.y < 19);
    }

    #[test]
    fn test_movement() {
        let mut game = GameState::new(20, 20);
        let initial_head = *game.snake().back().unwrap();
        
        game.update().unwrap();
        
        let new_head = game.snake().back().unwrap();
        assert_eq!(new_head.x, initial_head.x + 1);
        assert_eq!(new_head.y, initial_head.y);
        assert!(!game.is_game_over());
    }
}