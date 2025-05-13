// Remove unused Point import since we're only using tuples for positions
pub struct ObstaclePattern {
    pub positions: Vec<(u16, u16)>,
    pub sizes: Vec<(u16, u16)>,
}