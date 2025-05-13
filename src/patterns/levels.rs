use super::types::ObstaclePattern;

pub fn level_one_pattern(center_x: u16, center_y: u16) -> ObstaclePattern {
    let offset = 6;
    let positions = vec![
        // Center obstacle
        (center_x - 1, center_y - 1),
        // Top obstacle
        (center_x - 1, center_y - offset),
        // Bottom obstacle
        (center_x - 1, center_y + offset - 2),
        // Left obstacle
        (center_x - offset, center_y - 1),
        // Right obstacle
        (center_x + offset - 2, center_y - 1),
    ];

    // All obstacles are 2x2 in level 1
    let sizes = vec![(2, 2); 5];

    ObstaclePattern { positions, sizes }
}

pub fn level_two_pattern(width: u16, height: u16) -> ObstaclePattern {
    let quarter_width = width / 4;
    let quarter_height = height / 4;
    let middle_width = width / 2 - 1;
    let middle_height = height / 2 - 1;

    let positions = vec![
        // Corner obstacles
        (quarter_width, quarter_height),
        (width - quarter_width - 2, quarter_height),
        (quarter_width, height - quarter_height - 2),
        (width - quarter_width - 2, height - quarter_height - 2),
        // Middle edge obstacles
        (middle_width, quarter_height),
        (middle_width, height - quarter_height - 2),
        (quarter_width, middle_height),
        (width - quarter_width - 2, middle_height),
    ];

    // All obstacles are 2x2 in level 2
    let sizes = vec![(2, 2); 8];

    ObstaclePattern { positions, sizes }
}

pub fn level_three_pattern(center_x: u16, center_y: u16) -> ObstaclePattern {
    let inner_offset = 5;
    let outer_offset = 8;

    let positions = vec![
        // Inner cross
        (center_x - inner_offset, center_y - 1),
        (center_x + inner_offset - 2, center_y - 1),
        (center_x - 1, center_y - inner_offset),
        (center_x - 1, center_y + inner_offset - 2),
        // Outer diamond
        (center_x - outer_offset, center_y),
        (center_x + outer_offset - 2, center_y),
        (center_x - 1, center_y - outer_offset),
        (center_x - 1, center_y + outer_offset - 2),
        // Center obstacles
        (center_x - 3, center_y - 1),
        (center_x + 1, center_y - 1),
    ];

    // All obstacles are 2x2 in level 3
    let sizes = vec![(2, 2); 10];

    ObstaclePattern { positions, sizes }
}