use std::collections::VecDeque;
use crate::entities::{Point, Direction};

pub struct Snake {
    body: VecDeque<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new(start_x: u16, start_y: u16) -> Self {
        let mut body = VecDeque::new();
        
        // Initialize with 3 segments
        body.push_back(Point::new(start_x - 2, start_y));
        body.push_back(Point::new(start_x - 1, start_y));
        body.push_back(Point::new(start_x, start_y));

        Self {
            body,
            direction: Direction::Right,
        }
    }

    pub fn move_forward(&mut self, new_head: Point) {
        self.body.push_back(new_head);
    }

    pub fn retract_tail(&mut self) {
        self.body.pop_front();
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if new_direction != self.direction.opposite() {
            self.direction = new_direction;
        }
    }

    pub fn head(&self) -> Option<&Point> {
        self.body.back()
    }

    pub fn next_head_position(&self) -> Option<Point> {
        self.head().map(|head| head.translate(&self.direction))
    }

    pub fn body(&self) -> &VecDeque<Point> {
        &self.body
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn clear(&mut self) {
        self.body.clear();
    }
}