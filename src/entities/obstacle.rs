use std::collections::HashSet;
use super::Point;

#[derive(Clone, Debug)]
pub struct Obstacle {
    pub blocks: HashSet<Point>,
}

impl Obstacle {
    pub fn new_rectangle(top_left: Point, width: u16, height: u16) -> Self {
        let mut blocks = HashSet::new();
        
        for x in 0..width {
            for y in 0..height {
                blocks.insert(Point::new(
                    top_left.x + x,
                    top_left.y + y,
                ));
            }
        }
        
        Self { blocks }
    }

    pub fn collides_with(&self, point: &Point) -> bool {
        self.blocks.contains(point)
    }
}