use crate::{
    entities::{Point, Obstacle},
    utils::constants::BORDER_THICKNESS,
};
use std::collections::VecDeque;

pub struct CollisionManager {
    dimensions: (u16, u16),
}

impl CollisionManager {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            dimensions: (width, height),
        }
    }

    pub fn is_wall_collision(&self, point: &Point) -> bool {
        point.x < BORDER_THICKNESS || 
        point.x >= self.dimensions.0 - BORDER_THICKNESS || 
        point.y < BORDER_THICKNESS || 
        point.y >= self.dimensions.1 - BORDER_THICKNESS
    }

    pub fn is_self_collision(&self, snake: &VecDeque<Point>) -> bool {
        if let Some(head) = snake.back() {
            snake.iter().take(snake.len() - 1).any(|p| p == head)
        } else {
            false
        }
    }

    pub fn is_obstacle_collision(&self, point: &Point, obstacles: &[Obstacle]) -> bool {
        obstacles.iter().any(|obstacle| obstacle.collides_with(point))
    }

    pub fn check_valid_position(&self, point: &Point, snake: &VecDeque<Point>, obstacles: &[Obstacle]) -> bool {
        !self.is_wall_collision(point) && 
        !self.is_self_collision(snake) && 
        !self.is_obstacle_collision(point, obstacles)
    }
}