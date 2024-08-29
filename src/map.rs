use rand::Rng;

#[derive(Clone)]
pub enum Tile {
    Floor,
    Wall,
}

pub struct Map {
    width: u32,
    height: u32,
    data: Vec<Tile>,
}

impl Map {
    pub fn new(rng: &mut impl Rng, width: u32, height: u32) -> Self {
        let mut data = vec![Tile::Wall; (width * height) as usize];

        for y in 2..(height - 2) {
            for x in 2..(width - 2) {
                if x % 2 != 0 && y % 2 != 0 {
                    continue;
                }
                data[(x + y * width) as usize] = Tile::Floor;
            }
        }

        for y in (3..(height - 3)).step_by(2) {
            let x = 3u32;
            let is_horizontal: bool = rng.gen();
            if is_horizontal {
                let new_x: u32 = if rng.gen() { x + 1 } else { x - 1 };
                data[(new_x + y * width) as usize] = Tile::Wall;
            } else {
                let new_y: u32 = if rng.gen() { y + 1 } else { y - 1 };
                data[(x + new_y * width) as usize] = Tile::Wall;
            }
        }

        for x in (5..(width - 3)).step_by(2) {
            for y in (3..(height - 3)).step_by(2) {
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
        }
    }
}
