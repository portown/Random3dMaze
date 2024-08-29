#[derive(Clone, Copy)]
pub enum Direction {
    East,
    North,
    West,
    South,
}

pub struct Player {
    x: u32,
    y: u32,
    direction: Direction,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            x: 2,
            y: 2,
            direction: Direction::South,
        }
    }
}
