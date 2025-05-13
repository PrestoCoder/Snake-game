pub struct TransitionManager {
    message: String,
}

impl TransitionManager {
    pub fn new() -> Self {
        Self {
            message: String::new(),
        }
    }

    pub fn create_level_complete_message(&mut self, level: u32, score: u32) {
        self.message = format!(
            "Level {} Complete!\nScore: {}\nPress SPACE to continue",
            level,
            score
        );
    }

    pub fn create_victory_message(&mut self, score: u32, max_levels: u32) {
        self.message = format!(
            "VICTORY!\nFinal Score: {}\nAll {} Levels Complete!\nPress 'q' to quit",
            score,
            max_levels
        );
    }

    pub fn create_game_over_message(&mut self, score: u32, current_level: u32, max_levels: u32) {
        self.message = format!(
            "GAME OVER!\nFinal Score: {}\nLevel {} of {}\nPress 'q' to quit",
            score,
            current_level,
            max_levels
        );
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn clear(&mut self) {
        self.message.clear();
    }
}