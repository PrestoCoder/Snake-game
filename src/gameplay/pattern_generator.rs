// src/gameplay/pattern_generator.rs

pub struct ObstaclePattern {
    pub positions: Vec<(u16, u16)>,
    pub sizes: Vec<(u16, u16)>,
}

pub fn get_level_pattern(level: u32, width: u16, height: u16) -> ObstaclePattern {
    let obstacle_count = (level + 1).pow(2);  // Perfect square: 4, 9, 16, ...
    let grid_size = (level + 1) as u16;       // 2x2 for level 1, 3x3 for level 2, 4x4 for level 3, ...

    // Calculate margins to leave space around edges
    let margin = 4;
    let playable_width = width - 2 * margin;
    let playable_height = height - 2 * margin;

    // Calculate spacing between obstacles
    let spacing_x = playable_width / (grid_size + 1);  // +1 to create gaps at edges
    let spacing_y = playable_height / (grid_size + 1);

    let mut positions = Vec::new();
    let mut sizes = Vec::new();

    // Generate grid positions
    for row in 1..=grid_size {
        for col in 1..=grid_size {
            let x = margin + (spacing_x * col);
            let y = margin + (spacing_y * row);
            
            positions.push((x, y));
            sizes.push((2, 2));  // Fixed size for now
        }
    }

    ObstaclePattern {
        positions,
        sizes,
    }
}