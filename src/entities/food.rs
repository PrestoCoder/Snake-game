use super::Point;
use rand::Rng;
use crate::config::BORDER_THICKNESS;

pub struct Food {
    position: Point,
}

impl Food {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn generate_new(
        width: u16, 
        height: u16, 
        is_position_valid: impl Fn(&Point) -> bool
    ) -> Self {
        let mut rng = rand::thread_rng();
        
        loop {
            let position = Point::new(
                rng.gen_range(BORDER_THICKNESS..width - BORDER_THICKNESS),
                rng.gen_range(BORDER_THICKNESS..height - BORDER_THICKNESS),
            );
            
            if is_position_valid(&position) {
                return Self::new(position);
            }
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }
}