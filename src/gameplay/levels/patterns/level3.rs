use super::ObstaclePattern;

pub fn get_pattern(width: u16, height: u16) -> ObstaclePattern {
    let center_x = width / 2;
    let center_y = height / 2;
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