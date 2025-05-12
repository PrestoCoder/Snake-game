use crate::{
    error::{GameError, Result},
    types::{Direction, Point, Obstacle, GameEndReason, LevelState},
    config::Config,
};
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
    end_reason: Option<GameEndReason>,
    level_state: LevelState,
    base_obstacles: u32,
    obstacles_per_level: u32,
    obstacle_sizes: Vec<u16>,
}

impl GameState {
    pub fn new(config: &Config) -> Self {
        let mut snake = VecDeque::new();
        let center_x = config.width / 2;
        let center_y = config.height / 2;
        
        // Initialize snake with 3 segments
        snake.push_back(Point::new(center_x - 2, center_y));
        snake.push_back(Point::new(center_x - 1, center_y));
        snake.push_back(Point::new(center_x, center_y));

        let level_state = LevelState::new(
            config.starting_level,
            config.max_levels,
            config.score_per_level
        );

        let obstacles = Self::generate_obstacles(
            config.width,
            config.height,
            config.base_obstacles,
            config.obstacles_per_level,
            level_state.current_level,
            &config.obstacle_sizes,
        );

        let food = Self::generate_food_avoiding_all(config.width, config.height, &snake, &obstacles);

        Self {
            snake,
            food,
            direction: Direction::Right,
            score: 0,
            dimensions: (config.width, config.height),
            game_over: false,
            obstacles,
            speed_level: 1,
            end_reason: None,
            level_state,
            base_obstacles: config.base_obstacles,
            obstacles_per_level: config.obstacles_per_level,
            obstacle_sizes: config.obstacle_sizes.clone(),
        }
    }

    pub fn get_tick_rate(&self) -> u64 {
        let base_speed: u64 = 200;
        let speed_decrease: u64 = 10;
        let min_speed: u64 = 50;

        base_speed.saturating_sub(speed_decrease * (self.speed_level - 1) as u64)
            .max(min_speed)
    }

    fn generate_obstacles(
        width: u16,
        height: u16,
        base_count: u32,
        per_level: u32,
        current_level: u32,
        sizes: &[u16],
    ) -> Vec<Obstacle> {
        let mut obstacles: Vec<Obstacle> = Vec::new();  // Added type annotation
        let mut rng = rand::thread_rng();
        
        // Calculate total obstacles for current level
        let total_obstacles = base_count + (per_level * (current_level - 1));
        
        for _ in 0..total_obstacles {
            let size = sizes[rng.gen_range(0..sizes.len())];
            let mut valid = false;
            let mut attempts = 0;
            
            // Try to place obstacle in valid location
            while !valid && attempts < 100 {
                let x = rng.gen_range(width / 4..3 * width / 4);
                let y = rng.gen_range(height / 4..3 * height / 4);
                
                // Check if position is far enough from center and other obstacles
                let new_obstacle = Obstacle::new_rectangle(Point::new(x, y), size, size);
                valid = true;
                
                // Check distance from other obstacles
                'outer: for existing in &obstacles {
                    for point in &new_obstacle.blocks {
                        for other_point in &existing.blocks {
                            if manhattan_distance(point, other_point) < 3 {
                                valid = false;
                                break 'outer;
                            }
                        }
                    }
                }
                
                if valid {
                    obstacles.push(new_obstacle);
                    break;
                }
                
                attempts += 1;
            }
        }
        
        obstacles
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

        if self.is_wall_collision(&new_head) || 
           self.is_self_collision() ||
           Self::is_obstacle_collision(&new_head, &self.obstacles) {
            self.game_over = true;
            self.end_reason = Some(GameEndReason::Collision);
            return Ok(());
        }

        self.snake.push_back(new_head);

        if new_head == self.food {
            self.score += 1;
            self.speed_level += 1;

            // Check for level advancement
            if self.level_state.should_advance(self.score) {
                self.level_state.advance();
                
                // Generate new obstacles for the new level
                self.obstacles = Self::generate_obstacles(
                    self.dimensions.0,
                    self.dimensions.1,
                    self.base_obstacles,
                    self.obstacles_per_level,
                    self.level_state.current_level,
                    &self.obstacle_sizes,
                );
            }

            // Check for final victory (completed all levels)
            if self.level_state.current_level == self.level_state.max_levels &&
               self.score >= self.level_state.score_per_level * self.level_state.max_levels {
                self.game_over = true;
                self.end_reason = Some(GameEndReason::Victory);
                return Ok(());
            }

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
    pub fn end_reason(&self) -> Option<GameEndReason> { self.end_reason }
    pub fn current_level(&self) -> u32 { self.level_state.current_level }
    pub fn max_levels(&self) -> u32 { self.level_state.max_levels }
    pub fn score_needed_for_next(&self) -> Option<u32> { self.level_state.score_needed_for_next() }
}

// Helper function for obstacle placement
fn manhattan_distance(p1: &Point, p2: &Point) -> u16 {
    ((p1.x as i32 - p2.x as i32).abs() + (p1.y as i32 - p2.y as i32).abs()) as u16
}