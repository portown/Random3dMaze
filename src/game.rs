use crate::map::{Map, Tile};
use crate::player::Player;
use crate::render::{
    color_rgb, create_font, create_solid_brush, create_solid_pen, point, rect, rect_wh,
    BackSurface, Brush, Font, Pen, PrimarySurface, Surface,
};
use rand::{Rng, SeedableRng};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IOError(#[from] crate::render::Error),
}

struct RenderingData {
    map_surface: BackSurface,
    wall_surface: BackSurface,
    mini_map_surface: BackSurface,

    black_pen: Pen,
    black_brush: Brush,
    white_brush: Brush,
    start_brush: Brush,
    goal_brush: Brush,
}

pub struct Game {
    #[allow(unused)]
    rng_seed: <rand_chacha::ChaCha8Rng as rand::SeedableRng>::Seed,
    rng: rand_chacha::ChaCha8Rng,

    map: Map,
    player: Player,

    shows_mini_map: bool,

    mini_map_view_count: u32,
    key_press_count: u32,

    font: Font,
    rendering_data: Option<RenderingData>,
}

impl Game {
    pub fn new() -> Result<Self, Error> {
        let mut rng_seed = <rand_chacha::ChaCha8Rng as rand::SeedableRng>::Seed::default();
        rand::thread_rng().fill(&mut rng_seed);
        let mut rng = rand_chacha::ChaCha8Rng::from_seed(rng_seed);

        let map = Map::new(&mut rng, 23, 23);

        let font = create_font("MS UI Gothic", 20)?;

        Ok(Game {
            rng_seed,
            rng,

            map,
            player: Player::default(),

            shows_mini_map: false,

            mini_map_view_count: 0,
            key_press_count: 0,

            font,
            rendering_data: None,
        })
    }

    pub fn turn_left(&mut self) {
        use crate::player::Direction;
        self.player.direction = match self.player.direction {
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
        };
        self.key_press_count += 1;
    }

    pub fn turn_right(&mut self) {
        use crate::player::Direction;
        self.player.direction = match self.player.direction {
            Direction::West => Direction::North,
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
        };
        self.key_press_count += 1;
    }

    pub fn turn_back(&mut self) {
        use crate::player::Direction;
        self.player.direction = match self.player.direction {
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
        };
        self.key_press_count += 1;
    }

    pub fn move_forward(&mut self) {
        use crate::player::Direction;
        let point_diff = match self.player.direction {
            Direction::West => (-1, 0),
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
        };
        let Some(new_x) = self.player.x.checked_add_signed(point_diff.0) else {
            return;
        };
        if new_x >= self.map.width {
            return;
        }
        let Some(new_y) = self.player.y.checked_add_signed(point_diff.1) else {
            return;
        };
        if new_y >= self.map.height {
            return;
        }
        if self
            .map
            .get(new_x as i32, new_y as i32)
            .unwrap_or(Tile::Wall)
            == Tile::Wall
        {
            return;
        }

        self.player.x = new_x;
        self.player.y = new_y;
        self.key_press_count += 1;
    }

    pub fn toggle_mini_map(&mut self) {
        self.shows_mini_map = !self.shows_mini_map;
        self.mini_map_view_count += 1;
    }

    pub fn draw(&mut self, surface: &PrimarySurface) -> Result<(), Error> {
        let r = self
            .rendering_data
            .take()
            .map_or_else(|| self.create_rendering_data(surface), Ok)?;

        surface.draw_rect(&rect_wh(48 - 1, 48 - 1, 256 + 2, 256 + 2), &r.black_pen);
        self.draw_wall(&r);
        surface.copy_from(&rect_wh(48, 48, 256, 256), &r.map_surface, 0, 0);

        let mini_map_x = 48 * 2 + 256;
        let mini_map_y = 48;
        surface.draw_rect(
            &rect_wh(mini_map_x - 1, mini_map_y - 1, 256 + 2, 256 + 2),
            &r.black_pen,
        );
        if self.shows_mini_map {
            surface.copy_from(
                &rect_wh(mini_map_x, mini_map_y, 256, 256),
                &r.mini_map_surface,
                0,
                0,
            );
            let triangle = match self.player.direction {
                crate::player::Direction::West => vec![
                    point(
                        mini_map_x + (self.player.x * 67 / 6 + 10) as i32,
                        mini_map_y + (self.player.y * 67 / 6 + 10) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6) as i32,
                        mini_map_y + (self.player.y * 67 / 6 + 5) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6 + 10) as i32,
                        mini_map_y + (self.player.y * 67 / 6) as i32,
                    ),
                ],
                crate::player::Direction::North => vec![
                    point(
                        mini_map_x + (self.player.x * 67 / 6 + 5) as i32,
                        mini_map_y + (self.player.y * 67 / 6) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6) as i32,
                        mini_map_y + (self.player.y * 67 / 6 + 10) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6 + 10) as i32,
                        mini_map_y + (self.player.y * 67 / 6 + 10) as i32,
                    ),
                ],
                crate::player::Direction::East => vec![
                    point(
                        mini_map_x + (self.player.x * 67 / 6) as i32,
                        mini_map_y + (self.player.y * 67 / 6) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6 + 10) as i32,
                        mini_map_y + (self.player.y * 67 / 6 + 5) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6) as i32,
                        mini_map_y + (self.player.y * 67 / 6 + 10) as i32,
                    ),
                ],
                crate::player::Direction::South => vec![
                    point(
                        mini_map_x + (self.player.x * 67 / 6 + 5) as i32,
                        mini_map_y + (self.player.y * 67 / 6 + 10) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6) as i32,
                        mini_map_y + (self.player.y * 67 / 6) as i32,
                    ),
                    point(
                        mini_map_x + (self.player.x * 67 / 6 + 10) as i32,
                        mini_map_y + (self.player.y * 67 / 6) as i32,
                    ),
                ],
            };
            surface.draw_polygon(&triangle, &r.black_pen, &r.white_brush);
        } else {
            surface.fill_rect(&rect_wh(mini_map_x, mini_map_y, 256, 256), &r.white_brush);
        }

        surface.fill_rect(&rect_wh(20, 48 + 256 + 12, 300, 20), &r.white_brush);
        surface.draw_text(
            "移動：矢印キー マップ：Mキー 終了：ESCキー",
            20,
            48 + 256 + 12,
            &self.font,
            color_rgb(0, 0, 0),
        );

        self.rendering_data = Some(r);

        Ok(())
    }

    fn create_rendering_data(&self, surface: &PrimarySurface) -> Result<RenderingData, Error> {
        let map_surface = surface.create_surface(256, 256);
        let wall_surface = surface.load_bitmap("assets\\wall.bmp")?;
        let mini_map_surface = surface.create_surface(256, 256);
        let black_pen = create_solid_pen(1, color_rgb(0, 0, 0));
        let black_brush = create_solid_brush(color_rgb(0, 0, 0));
        let white_brush = create_solid_brush(color_rgb(255, 255, 255));
        let start_brush = create_solid_brush(color_rgb(0, 255, 255));
        let goal_brush = create_solid_brush(color_rgb(255, 0, 0));

        let r = RenderingData {
            map_surface,
            wall_surface,
            mini_map_surface,
            black_pen,
            black_brush,
            white_brush,
            start_brush,
            goal_brush,
        };

        self.draw_mini_map(&r);

        Ok(r)
    }

    fn draw_wall(&self, r: &RenderingData) {
        r.map_surface
            .copy_from(&rect_wh(0, 0, 256, 256), &r.wall_surface, 0, 0);

        let sight = self.player.sight(&self.map);

        // Forward 3
        if sight.get(3, -3) == Tile::Wall {
            r.map_surface.copy_from(
                &rect_wh(0, 108, 40 - 12, 40),
                &r.wall_surface,
                876 + 12,
                108,
            )
        }
        if sight.get(3, -2) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(28, 108, 40, 40), &r.wall_surface, 876, 108)
        }
        if sight.get(3, -1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(68, 108, 40, 40), &r.wall_surface, 876, 108)
        }
        if sight.get(3, 1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(148, 108, 40, 40), &r.wall_surface, 876, 108)
        }
        if sight.get(3, 2) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(188, 108, 40, 40), &r.wall_surface, 876, 108)
        }
        if sight.get(3, 3) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(228, 108, 256 - 228, 40), &r.wall_surface, 876, 108)
        }
        if sight.get(3, 0) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(108, 108, 40, 40), &r.wall_surface, 876, 108)
        }

        // Forward 2
        if sight.get(2, -2) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(0, 72, 21, 112), &r.wall_surface, 768, 72)
        }
        if sight.get(2, -1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(21, 72, 89, 112), &r.wall_surface, 788, 72)
        }
        if sight.get(2, 1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(147, 72, 89, 112), &r.wall_surface, 915, 72)
        }
        if sight.get(2, 2) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(235, 72, 21, 112), &r.wall_surface, 1003, 72)
        }
        if sight.get(2, 0) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(72, 72, 112, 112), &r.wall_surface, 584, 72)
        }

        // Forward 1
        if sight.get(1, -1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(0, 20, 73, 216), &r.wall_surface, 512, 20)
        }
        if sight.get(1, 1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(183, 20, 73, 216), &r.wall_surface, 695, 20)
        }
        if sight.get(1, 0) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(20, 20, 216, 216), &r.wall_surface, 276, 20)
        }

        // Forward 0
        if sight.get(0, -1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(0, 0, 20, 256), &r.wall_surface, 256, 0)
        }
        if sight.get(0, 1) == Tile::Wall {
            r.map_surface
                .copy_from(&rect_wh(236, 0, 20, 256), &r.wall_surface, 492, 0)
        }
    }

    fn draw_mini_map(&self, r: &RenderingData) {
        let surface_size = r.mini_map_surface.get_size();
        r.mini_map_surface.fill_rect(
            &rect_wh(0, 0, surface_size.0 as i32, surface_size.1 as i32),
            &r.white_brush,
        );

        let map_size = (self.map.width, self.map.height);
        let rect_at = |x: u32, y: u32| {
            rect(
                (x * surface_size.0 / map_size.0) as i32,
                (y * surface_size.1 / map_size.1) as i32,
                ((x + 1) * surface_size.0 / map_size.0) as i32,
                ((y + 1) * surface_size.1 / map_size.1) as i32,
            )
        };
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if let Some(Tile::Floor) = self.map.get(x as i32, y as i32) {
                    continue;
                }

                r.mini_map_surface.fill_rect(&rect_at(x, y), &r.black_brush);
            }
        }

        r.mini_map_surface.fill_rect(&rect_at(2, 2), &r.start_brush);
        r.mini_map_surface
            .fill_rect(&rect_at(map_size.0 - 3, map_size.1 - 3), &r.goal_brush);
    }
}

impl Drop for Game {
    fn drop(&mut self) {}
}
