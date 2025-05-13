use crate::{
    error::{GameError, Result},
    types::{Direction, Point, Obstacle, GameEndReason, GameState as GameStateEnum, LevelState},
    config::Config,
};
use rand::Rng;  // Add this import
use std::collections::VecDeque;

const BORDER_THICKNESS: u16 = 2;
const BASE_SPEED_LEVEL: u32 = 1;

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
            food: Point::new(0, 0),  // Temporary value
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
        // Reset snake position
        self.snake.clear();
        let center_x = self.dimensions.0 / 2;
        let center_y = self.dimensions.1 / 2;
        
        // Initialize snake with 3 segments
        self.snake.push_back(Point::new(center_x - 2, center_y));
        self.snake.push_back(Point::new(center_x - 1, center_y));
        self.snake.push_back(Point::new(center_x, center_y));

        // Reset direction
        self.direction = Direction::Right;

        // Reset speed for new level
        self.speed_level = BASE_SPEED_LEVEL;

        // Generate new obstacles
        self.obstacles = Self::generate_obstacles(
            self.dimensions.0,
            self.dimensions.1,
            self.base_obstacles,
            self.obstacles_per_level,
            self.level_state.current_level,
            &self.obstacle_sizes,
        );

        // Generate new food
        self.food = Self::generate_food_avoiding_all(
            self.dimensions.0,
            self.dimensions.1,
            &self.snake,
            &self.obstacles,
        );
    }

    fn generate_obstacles(
        width: u16,
        height: u16,
        _base_count: u32,
        _per_level: u32,
        current_level: u32,
        _sizes: &[u16],
    ) -> Vec<Obstacle> {
        let pattern = crate::patterns::get_level_pattern(current_level, width, height);
        let mut obstacles = Vec::new();

        // Create obstacles based on the pattern
        for ((x, y), (width, height)) in pattern.positions.iter().zip(pattern.sizes.iter()) {
            obstacles.push(Obstacle::new_rectangle(
                Point::new(*x, *y),
                *width,
                *height
            ));
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
           Self::is_obstacle_collision(&new_head, &self.obstacles) {
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
        let base_speed: u64 = 200;
        let speed_decrease: u64 = 10;
        let min_speed: u64 = 50;

        base_speed.saturating_sub(speed_decrease * (self.speed_level - 1) as u64)
            .max(min_speed)
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
    pub fn game_state(&self) -> GameStateEnum { self.state }
    pub fn obstacles(&self) -> &Vec<Obstacle> { &self.obstacles }
    pub fn speed_level(&self) -> u32 { self.speed_level }
    pub fn current_level(&self) -> u32 { self.level_state.current_level }
    pub fn max_levels(&self) -> u32 { self.level_state.max_levels }
    pub fn transition_message(&self) -> &str { &self.transition_message }
    pub fn score_needed_for_next(&self) -> Option<u32> { self.level_state.score_needed_for_next() }
}