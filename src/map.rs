use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub goal_x: u32,
    pub goal_y: u32,
    data: Vec<Tile>,
}

impl Map {
    pub fn new(rng: &mut impl Rng, width: u32, height: u32) -> Self {
        let mut data = vec![Tile::Wall; (width * height) as usize];

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                if x % 2 != 1 && y % 2 != 1 {
                    continue;
                }
                data[(x + y * width) as usize] = Tile::Floor;
            }
        }

        for y in (2..(height - 2)).step_by(2) {
            let x = 2u32;
            let is_horizontal: bool = rng.gen();
            if is_horizontal {
                let new_x: u32 = if rng.gen() { x + 1 } else { x - 1 };
                data[(new_x + y * width) as usize] = Tile::Wall;
            } else {
                let new_y: u32 = if rng.gen() { y + 1 } else { y - 1 };
                data[(x + new_y * width) as usize] = Tile::Wall;
            }
        }

        for x in (4..(width - 2)).step_by(2) {
            for y in (2..(height - 2)).step_by(2) {
                let is_horizontal: bool = rng.gen();
                if is_horizontal {
                    let new_x: u32 = x + 1;
                    data[(new_x + y * width) as usize] = Tile::Wall;
                } else {
                    let new_y: u32 = if rng.gen() { y + 1 } else { y - 1 };
                    data[(x + new_y * width) as usize] = Tile::Wall;
                }
            }
        }

        Map {
            width,
            height,
            data,
            start_x: 1,
            start_y: 1,
            goal_x: width - 2,
            goal_y: height - 2,
        }
    }

    fn index_of(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x as u32 >= self.width {
            return None;
        }
        if y < 0 || y as u32 >= self.height {
            return None;
        }
        Some(((x as u32) + (y as u32) * self.width) as usize)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Tile> {
        self.index_of(x, y).map(|i| self.data[i].clone())
    }
}
