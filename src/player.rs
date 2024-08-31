#[derive(Clone, Copy)]
pub enum Direction {
    West,
    North,
    East,
    South,
}

pub struct Player {
    pub x: u32,
    pub y: u32,
    pub direction: Direction,
}

pub struct Sight<'a, 'b> {
    map: &'a crate::map::Map,
    player: &'b Player,
}

impl Player {
    pub fn sight<'a, 'b>(&'b self, map: &'a crate::map::Map) -> Sight<'a, 'b> {
        Sight { map, player: &self }
    }
}

impl<'a, 'b> Sight<'a, 'b> {
    pub fn get(&self, forward: u32, horizontal: i32) -> crate::map::Tile {
        let (x_diff, y_diff) = match self.player.direction {
            Direction::West => (-(forward as i32), -horizontal),
            Direction::North => (horizontal, -(forward as i32)),
            Direction::East => (forward as i32, horizontal),
            Direction::South => (-horizontal, forward as i32),
        };

        self.map.get(
            self.player.x.saturating_add_signed(x_diff) as i32,
            self.player.y.saturating_add_signed(y_diff) as i32,
        ).unwrap_or(crate::map::Tile::Wall)
    }
}
