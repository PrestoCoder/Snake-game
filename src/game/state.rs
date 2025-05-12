use crate::{
    error::{GameError, Result},
    types::{Direction, Point, Obstacle},
};
use log::debug;
use std::collections::VecDeque;
use rand::Rng;

const BORDER_THICKNESS: u16 = 2;

pub struct GameState {
    snake: VecDeque<Point>,
    food: Point,
    direction: Direction,
    score: u32,
    dimensions: (u16, u16),
    game_over: bool,
    obstacles: Vec<Obstacle>,
    speed_level: u32,
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

        let obstacles = Self::generate_obstacles(width, height);
        let food = Self::generate_food_avoiding_all(width, height, &snake, &obstacles);

        Self {
            snake,
            food,
            direction: Direction::Right,
            score: 0,
            dimensions: (width, height),
            game_over: false,
            obstacles,
            speed_level: 1,
        }
    }

    fn generate_obstacles(width: u16, height: u16) -> Vec<Obstacle> {
        let mut obstacles = Vec::new();
        
        // Left side obstacles
        obstacles.push(Obstacle::new_rectangle(
            Point::new(width / 4, height / 4),
            2,
            2,
        ));

        // Right side obstacles
        obstacles.push(Obstacle::new_rectangle(
            Point::new(3 * width / 4, height / 4),
            2,
            2,
        ));

        // Top center
        obstacles.push(Obstacle::new_rectangle(
            Point::new(width / 2 - 1, height / 4),
            2,
            2,
        ));

        // Bottom center
        obstacles.push(Obstacle::new_rectangle(
            Point::new(width / 2 - 1, 3 * height / 4),
            2,
            2,
        ));

        // Add some small single-block obstacles
        obstacles.push(Obstacle::new_rectangle(
            Point::new(width / 3, height / 2),
            1,
            1,
        ));

        obstacles.push(Obstacle::new_rectangle(
            Point::new(2 * width / 3, height / 2),
            1,
            1,
        ));

        obstacles
    }

    pub fn get_tick_rate(&self) -> u64 {
        // Start at 200ms and decrease with each level
        // But don't go below 50ms to keep the game playable
        let base_speed: u64 = 200;
        let speed_decrease: u64 = 10;  // Decrease by 10ms per level
        let min_speed: u64 = 50;

        base_speed.saturating_sub(speed_decrease * (self.speed_level - 1) as u64)
            .max(min_speed)
    }

    fn generate_food_avoiding_all(
        width: u16, 
        height: u16, 
        snake: &VecDeque<Point>,
        obstacles: &[Obstacle],
    ) -> Point {
        let mut rng = rand::thread_rng();
        
        loop {
            let food = Point::new(
                rng.gen_range(BORDER_THICKNESS..width - BORDER_THICKNESS),
                rng.gen_range(BORDER_THICKNESS..height - BORDER_THICKNESS),
            );
            
            if !snake.contains(&food) && !Self::is_obstacle_collision(&food, obstacles) {
                return food;
            }
        }
    }

    fn is_obstacle_collision(point: &Point, obstacles: &[Obstacle]) -> bool {
        obstacles.iter().any(|obstacle| obstacle.collides_with(point))
    }

    pub fn update(&mut self) -> Result<()> {
        if self.game_over {
            return Ok(());
        }

        let current_head = self.snake.back()
            .ok_or_else(|| GameError::GameState("Snake has no head".to_string()))?;
        
        let new_head = current_head.translate(&self.direction);

        debug!("Current head: ({}, {}), New head: ({}, {}), Direction: {:?}", 
            current_head.x, current_head.y, new_head.x, new_head.y, self.direction);

        if self.is_wall_collision(&new_head) || 
           self.is_self_collision() ||
           Self::is_obstacle_collision(&new_head, &self.obstacles) {
            debug!("Collision detected at ({}, {})", new_head.x, new_head.y);
            self.game_over = true;
            return Ok(());
        }

        self.snake.push_back(new_head);

        if new_head == self.food {
            debug!("Food eaten at ({}, {})", self.food.x, self.food.y);
            self.score += 1;
            self.speed_level += 1;
            self.food = Self::generate_food_avoiding_all(
                self.dimensions.0,
                self.dimensions.1,
                &self.snake,
                &self.obstacles,
            );
        } else {
            self.snake.pop_front();
        }

        Ok(())
    }

    pub fn is_wall_collision(&self, point: &Point) -> bool {
        point.x < BORDER_THICKNESS || 
        point.x >= self.dimensions.0 - BORDER_THICKNESS || 
        point.y < BORDER_THICKNESS || 
        point.y >= self.dimensions.1 - BORDER_THICKNESS
    }

    fn is_self_collision(&self) -> bool {
        if let Some(head) = self.snake.back() {
            self.snake.iter().take(self.snake.len() - 1).any(|p| p == head)
        } else {
            false
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if new_direction != self.direction.opposite() {
            self.direction = new_direction;
        }
    }

    // Getters
    pub fn snake(&self) -> &VecDeque<Point> { &self.snake }
    pub fn food(&self) -> &Point { &self.food }
    pub fn score(&self) -> u32 { self.score }
    pub fn is_game_over(&self) -> bool { self.game_over }
    pub fn obstacles(&self) -> &Vec<Obstacle> { &self.obstacles }
    pub fn speed_level(&self) -> u32 { self.speed_level }
}