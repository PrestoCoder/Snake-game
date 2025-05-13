use crate::{
    utils::{Result, GameError, constants::*},
    entities::{Direction, Point, Obstacle, Food},
    gameplay::{GameState as GameStateEnum, GameEndReason, Snake, levels::{LevelState, get_level_pattern}},
    config::Config,
};

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
    pub fn new(config: &Config) -> Self {
        let dimensions = (config.width, config.height);
        let level_state = LevelState::new(
            config.starting_level,
            config.max_levels,
            config.score_per_level
        );

        let mut state = Self {
            snake: Snake::new(config.width / 2, config.height / 2),
            food: Food::new(Point::new(0, 0)),  // Temporary
            score_manager: super::ScoreManager::new(),
            collision_manager: super::CollisionManager::new(config.width, config.height),
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
        let center_x = self.dimensions.0 / 2;
        let center_y = self.dimensions.1 / 2;
        
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

        // Generate food in valid position
        self.food = Food::generate_new(
            self.dimensions.0,
            self.dimensions.1,
            |point| {
                !self.snake.body().contains(point) && 
                !self.collision_manager.is_obstacle_collision(point, &self.obstacles)
            }
        );
    }

    pub fn update(&mut self) -> Result<()> {
        match self.state {
            GameStateEnum::Playing => self.update_playing(),
            GameStateEnum::LevelTransition => Ok(()),
            GameStateEnum::GameOver(_) => Ok(()),
        }
    }

    fn update_playing(&mut self) -> Result<()> {
        let new_head = self.snake.next_head_position()
            .ok_or_else(|| GameError::GameState("Snake has no head".to_string()))?;

        if self.collision_manager.is_wall_collision(&new_head) || 
           self.collision_manager.is_self_collision(self.snake.body()) ||
           self.collision_manager.is_obstacle_collision(&new_head, &self.obstacles) {
            self.state = GameStateEnum::GameOver(GameEndReason::Collision);
            return Ok(());
        }

        self.snake.move_forward(new_head);

        if new_head == *self.food.position() {
            self.score_manager.add_score(1);

            if self.level_state.should_advance(self.score_manager.score()) {
                self.level_state.advance();
                self.prepare_next_level();
                return Ok(());
            }

            if self.level_state.current_level == self.level_state.max_levels &&
               self.score_manager.score() >= self.level_state.score_per_level * self.level_state.max_levels {
                self.state = GameStateEnum::GameOver(GameEndReason::Victory);
                return Ok(());
            }

            self.food = Food::generate_new(
                self.dimensions.0,
                self.dimensions.1,
                |point| {
                    !self.snake.body().contains(point) && 
                    !self.collision_manager.is_obstacle_collision(point, &self.obstacles)
                }
            );
        } else {
            self.snake.retract_tail();
        }

        Ok(())
    }

    fn prepare_next_level(&mut self) {
        self.state = GameStateEnum::LevelTransition;
        self.transition_message = format!(
            "Level {} Complete!\nScore: {}\nPress SPACE to continue",
            self.level_state.current_level - 1,
            self.score_manager.score()
        );
    }

    pub fn start_next_level(&mut self) {
        self.reset_level();
        self.state = GameStateEnum::Playing;
    }

    pub fn get_tick_rate(&self) -> u64 {
        BASE_TICK_RATE.saturating_sub(SPEED_DECREASE_PER_LEVEL * (self.score_manager.speed_level() - 1))
            .max(MIN_SPEED)
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