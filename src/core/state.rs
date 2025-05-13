// src/core/state.rs
use crate::{
    utils::{Result, GameError},
    entities::{Direction, Point, Obstacle, Food},
    gameplay::{
        GameState as GameStateEnum, 
        GameEndReason, 
        Snake, 
        LevelState,
        get_level_pattern
    },
    config::*,
};
use log::debug;

pub struct GameState {
    snake: Snake,
    food: Food,
    score_manager: super::ScoreManager,
    collision_manager: super::CollisionManager,
    dimensions: (u16, u16),
    state: GameStateEnum,
    obstacles: Vec<Obstacle>,
    level_state: LevelState,
    transition_message: String,
}

impl GameState {
    pub fn new() -> Self {
        let dimensions = (WIDTH, HEIGHT);
        let level_state = LevelState::new(
            STARTING_LEVEL,
            MAX_LEVELS,
            SCORE_PER_LEVEL
        );

        debug!("Initializing game with dimensions: {}x{}", dimensions.0, dimensions.1);
        debug!("Level settings - Start: {}, Max: {}, Score per level: {}", 
            STARTING_LEVEL, MAX_LEVELS, SCORE_PER_LEVEL);

        let mut state = Self {
            snake: Snake::new(WIDTH / 10, HEIGHT / 10),  // Changed to start at 1/10th of screen
            food: Food::new(Point::new(0, 0)),
            score_manager: super::ScoreManager::new(),
            collision_manager: super::CollisionManager::new(WIDTH, HEIGHT),
            dimensions,
            state: GameStateEnum::Playing,
            obstacles: Vec::new(),
            level_state,
            transition_message: String::new(),
        };

        state.reset_level();
        state
    }

    fn reset_level(&mut self) {
        debug!("Resetting level {}", self.level_state.current_level);
        let center_x = self.dimensions.0 / 10;
        let center_y = self.dimensions.1 / 10;
        
        self.snake = Snake::new(center_x, center_y);
        self.score_manager.reset_speed();

        // Generate new obstacles using pattern
        let pattern = get_level_pattern(
            self.level_state.current_level,
            self.dimensions.0,
            self.dimensions.1
        );

        self.obstacles = pattern.positions.iter().zip(pattern.sizes.iter())
            .map(|((x, y), (w, h))| Obstacle::new_rectangle(Point::new(*x, *y), *w, *h))
            .collect();

        debug!("Generated {} obstacles for level {}", self.obstacles.len(), self.level_state.current_level);

        // Generate food in valid position
        self.generate_new_food();
    }

    fn generate_new_food(&mut self) {
        self.food = Food::generate_new(
            self.dimensions.0,
            self.dimensions.1,
            |point| {
                !self.snake.body().contains(point) && 
                !self.collision_manager.is_obstacle_collision(point, &self.obstacles)
            }
        );
        debug!("New food generated at position: ({}, {})", 
            self.food.position().x, 
            self.food.position().y);
    }

    pub fn update(&mut self) -> Result<()> {
        match self.state {
            GameStateEnum::Playing => self.update_playing(),
            GameStateEnum::LevelTransition => Ok(()),
            GameStateEnum::GameOver(_) => Ok(()),
        }
    }

    fn update_playing(&mut self) -> Result<()> {
        let next_head = self.snake.next_head_position()
            .ok_or_else(|| GameError::GameState("Snake has no head".to_string()))?;

        // Check collisions first
        if self.collision_manager.is_wall_collision(&next_head) || 
           self.collision_manager.is_self_collision(self.snake.body()) ||
           self.collision_manager.is_obstacle_collision(&next_head, &self.obstacles) {
            debug!("Collision detected - Game Over");
            self.state = GameStateEnum::GameOver(GameEndReason::Collision);
            return Ok(());
        }

        // Check if we will collect food before moving
        let will_collect_food = next_head == *self.food.position();

        // Move snake
        self.snake.move_forward(next_head);

        // Handle food collection after movement
        if will_collect_food {
            self.score_manager.add_score(1);
            debug!("Food collected! Score: {}, Level: {}", 
                self.score_manager.score(), 
                self.level_state.current_level);

            // Check level advancement
            if self.level_state.should_advance(self.score_manager.score()) {
                debug!("Level {} complete! Advancing to next level", self.level_state.current_level);
                self.level_state.advance();
                self.prepare_next_level();
                return Ok(());
            }

            // Check victory condition
            if self.level_state.current_level == self.level_state.max_levels {
                let final_score_needed = self.level_state.score_per_level * self.level_state.max_levels;
                if self.score_manager.score() >= final_score_needed {
                    debug!("Final level complete! Victory!");
                    self.state = GameStateEnum::GameOver(GameEndReason::Victory);
                    return Ok(());
                }
            }

            // Generate new food
            self.generate_new_food();
        } else {
            self.snake.retract_tail();
        }

        Ok(())
    }

    fn prepare_next_level(&mut self) {
        debug!("Preparing level {} transition", self.level_state.current_level);
        self.state = GameStateEnum::LevelTransition;
        self.transition_message = format!(
            "Level {} Complete!\nScore: {}\nPress SPACE to continue",
            self.level_state.current_level - 1,
            self.score_manager.score()
        );
    }

    pub fn start_next_level(&mut self) {
        debug!("Starting level {}", self.level_state.current_level);
        self.reset_level();
        self.state = GameStateEnum::Playing;
    }

    pub fn get_tick_rate(&self) -> u64 {
        BASE_TICK_RATE.saturating_sub(
            SPEED_DECREASE_PER_LEVEL * u64::from(self.score_manager.speed_level() - 1)
        ).max(MIN_SPEED)
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        self.snake.change_direction(new_direction);
    }

    // Getters
    pub fn snake(&self) -> &Snake { &self.snake }
    pub fn food(&self) -> &Food { &self.food }
    pub fn score(&self) -> u32 { self.score_manager.score() }
    pub fn game_state(&self) -> GameStateEnum { self.state }
    pub fn obstacles(&self) -> &Vec<Obstacle> { &self.obstacles }
    pub fn speed_level(&self) -> u32 { self.score_manager.speed_level() }
    pub fn current_level(&self) -> u32 { self.level_state.current_level }
    pub fn max_levels(&self) -> u32 { self.level_state.max_levels }
    pub fn transition_message(&self) -> &str { &self.transition_message }
    pub fn score_needed_for_next(&self) -> Option<u32> { self.level_state.score_needed_for_next() }
}