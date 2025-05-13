use std::collections::VecDeque;
use rand::Rng;

use crate::{
    utils::{Result, GameError, constants::*},
    entities::{Direction, Point, Obstacle},
    gameplay::{GameState as GameStateEnum, GameEndReason, levels::{LevelState, get_level_pattern}},
    config::Config,
};

pub struct GameState {
    snake: VecDeque<Point>,
    food: Point,
    direction: Direction,
    score: u32,
    dimensions: (u16, u16),
    state: GameStateEnum,
    obstacles: Vec<Obstacle>,
    speed_level: u32,
    level_state: LevelState,
    base_obstacles: u32,
    obstacles_per_level: u32,
    obstacle_sizes: Vec<u16>,
    transition_message: String,
}

impl GameState {
    pub fn new(config: &Config) -> Self {
        let level_state = LevelState::new(
            config.starting_level,
            config.max_levels,
            config.score_per_level
        );

        let mut state = Self {
            snake: VecDeque::new(),
            food: Point::new(0, 0),
            direction: Direction::Right,
            score: 0,
            dimensions: (config.width, config.height),
            state: GameStateEnum::Playing,
            obstacles: Vec::new(),
            speed_level: BASE_SPEED_LEVEL,
            level_state,
            base_obstacles: config.base_obstacles,
            obstacles_per_level: config.obstacles_per_level,
            obstacle_sizes: config.obstacle_sizes.clone(),
            transition_message: String::new(),
        };

        state.reset_level();
        state
    }

    fn reset_level(&mut self) {
        self.snake.clear();
        let center_x = self.dimensions.0 / 2;
        let center_y = self.dimensions.1 / 2;
        
        self.snake.push_back(Point::new(center_x - 2, center_y));
        self.snake.push_back(Point::new(center_x - 1, center_y));
        self.snake.push_back(Point::new(center_x, center_y));

        self.direction = Direction::Right;
        self.speed_level = BASE_SPEED_LEVEL;

        // Generate new obstacles using pattern
        let pattern = get_level_pattern(
            self.level_state.current_level,
            self.dimensions.0,
            self.dimensions.1
        );

        self.obstacles = pattern.positions.iter().zip(pattern.sizes.iter())
            .map(|((x, y), (w, h))| Obstacle::new_rectangle(Point::new(*x, *y), *w, *h))
            .collect();

        self.food = self.generate_food();
    }

    fn generate_food(&self) -> Point {
        let mut rng = rand::thread_rng();
        
        loop {
            let food = Point::new(
                rng.gen_range(BORDER_THICKNESS..self.dimensions.0 - BORDER_THICKNESS),
                rng.gen_range(BORDER_THICKNESS..self.dimensions.1 - BORDER_THICKNESS),
            );
            
            if !self.snake.contains(&food) && !self.is_obstacle_collision(&food) {
                return food;
            }
        }
    }

    fn is_obstacle_collision(&self, point: &Point) -> bool {
        self.obstacles.iter().any(|obstacle| obstacle.collides_with(point))
    }

    pub fn update(&mut self) -> Result<()> {
        match self.state {
            GameStateEnum::Playing => self.update_playing(),
            GameStateEnum::LevelTransition => Ok(()),
            GameStateEnum::GameOver(_) => Ok(()),
        }
    }

    fn update_playing(&mut self) -> Result<()> {
        let current_head = self.snake.back()
            .ok_or_else(|| GameError::GameState("Snake has no head".to_string()))?;
        
        let new_head = current_head.translate(&self.direction);

        if self.is_wall_collision(&new_head) || 
           self.is_self_collision() ||
           self.is_obstacle_collision(&new_head) {
            self.state = GameStateEnum::GameOver(GameEndReason::Collision);
            return Ok(());
        }

        self.snake.push_back(new_head);

        if new_head == self.food {
            self.score += 1;
            self.speed_level += 1;

            if self.level_state.should_advance(self.score) {
                self.level_state.advance();
                self.prepare_next_level();
                return Ok(());
            }

            if self.level_state.current_level == self.level_state.max_levels &&
               self.score >= self.level_state.score_per_level * self.level_state.max_levels {
                self.state = GameStateEnum::GameOver(GameEndReason::Victory);
                return Ok(());
            }

            self.food = self.generate_food();
        } else {
            self.snake.pop_front();
        }

        Ok(())
    }

    fn prepare_next_level(&mut self) {
        self.state = GameStateEnum::LevelTransition;
        self.transition_message = format!(
            "Level {} Complete!\nScore: {}\nPress SPACE to continue",
            self.level_state.current_level - 1,
            self.score
        );
    }

    pub fn start_next_level(&mut self) {
        self.reset_level();
        self.state = GameStateEnum::Playing;
    }

    pub fn get_tick_rate(&self) -> u64 {
        BASE_TICK_RATE.saturating_sub(SPEED_DECREASE_PER_LEVEL * (self.speed_level - 1))
            .max(MIN_SPEED)
    }

    fn is_wall_collision(&self, point: &Point) -> bool {
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
    pub fn game_state(&self) -> GameStateEnum { self.state }
    pub fn obstacles(&self) -> &Vec<Obstacle> { &self.obstacles }
    pub fn speed_level(&self) -> u32 { self.speed_level }
    pub fn current_level(&self) -> u32 { self.level_state.current_level }
    pub fn max_levels(&self) -> u32 { self.level_state.max_levels }
    pub fn transition_message(&self) -> &str { &self.transition_message }
    pub fn score_needed_for_next(&self) -> Option<u32> { self.level_state.score_needed_for_next() }
}