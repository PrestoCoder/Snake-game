// src/gameplay/level_state.rs
use log::debug;

pub struct LevelState {
    pub current_level: u32,
    pub max_levels: u32,
    pub score_per_level: u32,
}

impl LevelState {
    pub fn new(starting_level: u32, max_levels: u32, score_per_level: u32) -> Self {
        Self {
            current_level: starting_level,
            max_levels,
            score_per_level,
        }
    }

    pub fn should_advance(&self, score: u32) -> bool {
        let score_needed = self.score_per_level * self.current_level;
        debug!("Checking level advance - Score: {}, Needed: {}, Current Level: {}, Max Levels: {}", 
            score, score_needed, self.current_level, self.max_levels);
        
        if score >= score_needed && self.current_level < self.max_levels {
            debug!("Should advance to next level");
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) {
        if self.current_level < self.max_levels {
            self.current_level += 1;
            debug!("Advanced to level {}", self.current_level);
        }
    }

    pub fn score_needed_for_next(&self) -> Option<u32> {
        if self.current_level < self.max_levels {
            Some(self.score_per_level * self.current_level)
        } else {
            None
        }
    }
}