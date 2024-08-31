use crate::map::{Map, Tile};
use crate::player::Player;
use crate::render::{
    color_rgb, point, rect, rect_wh, Bitmap, Brush, Context, Font, Geometry, ImageLoader,
    RenderTarget,
};
use rand::{Rng, SeedableRng};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GeneralRenderingError(#[from] crate::render::Error),
    #[error("An error at the end of drawing")]
    EndDrawError,
}

struct RenderingData {
    map_surface: RenderTarget,
    wall_surface: Bitmap,
    mini_map_surface: RenderTarget,

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
    is_goal: bool,
    score: u32,

    mini_map_view_count: u32,
    key_press_count: u32,

    drew_mini_map: bool,
    player_geometry: Geometry,
    font: Font,
    rendering_data: Option<RenderingData>,
    image_loader: ImageLoader,
}

impl Game {
    pub fn new(render_context: &Context) -> Result<Self, Error> {
        let mut rng_seed = <rand_chacha::ChaCha8Rng as rand::SeedableRng>::Seed::default();
        rand::thread_rng().fill(&mut rng_seed);
        let mut rng = rand_chacha::ChaCha8Rng::from_seed(rng_seed);

        let map = Map::new(&mut rng, 21, 21);
        let player = Player {
            x: map.start_x,
            y: map.start_y,
            direction: crate::player::Direction::South,
        };

        let player_geometry = render_context.create_geometry(|p| {
            p.begin_figure(&point(0, (256 / map.height / 2) as i32));
            p.add_line(&point((256 / map.width) as i32, 0));
            p.add_line(&point((256 / map.width) as i32, (256 / map.height) as i32));
            p.end_figure();
        })?;

        let font = render_context.create_font("MS UI Gothic", 20)?;

        Ok(Game {
            rng_seed,
            rng,

            map,
            player,

            shows_mini_map: false,
            is_goal: false,
            score: 0,

            mini_map_view_count: 0,
            key_press_count: 0,

            drew_mini_map: false,
            player_geometry,
            font,
            rendering_data: None,

            image_loader: ImageLoader::new()?,
        })
    }

    pub fn turn_left(&mut self) {
        if self.is_goal {
            return;
        }
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
        if self.is_goal {
            return;
        }
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
        if self.is_goal {
            return;
        }
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
        if self.is_goal {
            return;
        }
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

        if new_x == self.map.goal_x && new_y == self.map.goal_y {
            self.is_goal = true;
            self.score = 5000 / self.key_press_count
                + if self.mini_map_view_count == 0 {
                    50
                } else {
                    10 / self.mini_map_view_count
                }
                + self.rng.gen_range(0..30);
        }
    }

    pub fn toggle_mini_map(&mut self) {
        if self.is_goal {
            return;
        }
        self.shows_mini_map = !self.shows_mini_map;
        self.mini_map_view_count += 1;
    }

    pub fn new_game(&mut self, render_context: &Context) -> Result<Game, Error> {
        let mut new = Game::new(render_context)?;
        new.rendering_data = self.rendering_data.take();
        Ok(new)
    }

    pub fn draw(&mut self, rt: &RenderTarget) -> Result<(), Error> {
        let r = self
            .rendering_data
            .take()
            .map_or_else(|| self.create_rendering_data(rt), Ok)?;

        rt.begin();

        rt.clear(color_rgb(255, 255, 255));

        rt.draw_rect(&rect_wh(48 - 1, 48 - 1, 256 + 2, 256 + 2), &r.black_brush);
        self.draw_wall(&r);
        rt.copy_from(
            &rect_wh(48, 48, 256, 256),
            &r.map_surface.get_bitmap()?,
            0,
            0,
        );

        let mini_map_x = 48 * 2 + 256;
        let mini_map_y = 48;
        rt.draw_rect(
            &rect_wh(mini_map_x - 1, mini_map_y - 1, 256 + 2, 256 + 2),
            &r.black_brush,
        );
        if self.shows_mini_map {
            if !self.drew_mini_map {
                self.draw_mini_map(&r);
            }
            rt.copy_from(
                &rect_wh(mini_map_x, mini_map_y, 256, 256),
                &r.mini_map_surface.get_bitmap()?,
                0,
                0,
            );
            let angle = match self.player.direction {
                crate::player::Direction::West => 0.0,
                crate::player::Direction::North => 90.0,
                crate::player::Direction::East => 180.0,
                crate::player::Direction::South => 270.0,
            };
            rt.draw_polygon(
                &self.player_geometry,
                mini_map_x + (self.player.x * 256 / self.map.width) as i32,
                mini_map_y + (self.player.y * 256 / self.map.height) as i32,
                &r.black_brush,
                &r.white_brush,
                angle,
            );
        } else {
            rt.fill_rect(&rect_wh(mini_map_x, mini_map_y, 256, 256), &r.white_brush);
        }

        rt.fill_rect(
            &rect_wh(0, 48 + 256 + 12, 48 * 3 + 256 * 2, 20),
            &r.white_brush,
        );
        if self.is_goal {
            let text = format!(
                "ゴール！　スコア：{}点　リスタート：Enterキー　終了：ESCキー",
                self.score
            );
            rt.draw_text(&text, 20, 48 + 256 + 12, &self.font, &r.black_brush);
        } else {
            rt.draw_text(
                "移動：矢印キー マップ：Mキー 終了：ESCキー",
                20,
                48 + 256 + 12,
                &self.font,
                &r.black_brush,
            );
        }

        if rt.end() {
            self.rendering_data = Some(r);
        } else {
            self.rendering_data = None;
            return Err(Error::EndDrawError);
        }

        Ok(())
    }

    fn create_rendering_data(&mut self, rt: &RenderTarget) -> Result<RenderingData, Error> {
        let map_surface = rt.create_new_render_target(256, 256)?;
        let wall_surface = self.image_loader.load_bitmap("assets\\wall.bmp", rt)?;
        let mini_map_surface = rt.create_new_render_target(256, 256)?;
        let black_brush = rt.create_solid_brush(color_rgb(0, 0, 0))?;
        let white_brush = rt.create_solid_brush(color_rgb(255, 255, 255))?;
        let start_brush = rt.create_solid_brush(color_rgb(0, 255, 255))?;
        let goal_brush = rt.create_solid_brush(color_rgb(255, 0, 0))?;

        self.drew_mini_map = false;

        Ok(RenderingData {
            map_surface,
            wall_surface,
            mini_map_surface,
            black_brush,
            white_brush,
            start_brush,
            goal_brush,
        })
    }

    fn draw_wall(&self, r: &RenderingData) {
        r.map_surface.begin();
        r.map_surface.clear(color_rgb(0, 0, 0));

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

        r.map_surface.end();
    }

    fn draw_mini_map(&mut self, r: &RenderingData) {
        r.mini_map_surface.begin();
        r.mini_map_surface.clear(color_rgb(255, 255, 255));
        let surface_size = r.mini_map_surface.get_size();

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

        r.mini_map_surface
            .fill_rect(&rect_at(self.map.start_x, self.map.start_y), &r.start_brush);
        r.mini_map_surface
            .fill_rect(&rect_at(self.map.goal_x, self.map.goal_y), &r.goal_brush);

        r.mini_map_surface.end();

        self.drew_mini_map = true;
    }
}

impl Drop for Game {
    fn drop(&mut self) {}
}
