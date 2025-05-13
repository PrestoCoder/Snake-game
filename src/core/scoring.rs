pub struct ScoreManager {
    score: u32,
    speed_level: u32,
}

impl ScoreManager {
    pub fn new() -> Self {
        Self {
            score: 0,
            speed_level: 1,
        }
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
        self.speed_level += 1;
    }

    pub fn reset_speed(&mut self) {
        self.speed_level = 1;
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn speed_level(&self) -> u32 {
        self.speed_level
    }
}