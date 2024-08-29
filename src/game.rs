use crate::map::Map;
use crate::player::Player;
use crate::render::{
    color_rgb, create_solid_brush, create_solid_pen, rect_wh, BackSurface, Brush, Pen,
    PrimarySurface, Surface,
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
}

pub struct Game {
    #[allow(unused)]
    rng_seed: <rand_chacha::ChaCha8Rng as rand::SeedableRng>::Seed,
    rng: rand_chacha::ChaCha8Rng,

    map: Map,
    player: Player,

    shows_mini_map: bool,

    map_view_count: u32,
    key_press_count: u32,

    rendering_data: Option<RenderingData>,
}

impl Game {
    pub fn new() -> Result<Self, Error> {
        let mut rng_seed = <rand_chacha::ChaCha8Rng as rand::SeedableRng>::Seed::default();
        rand::thread_rng().fill(&mut rng_seed);
        let mut rng = rand_chacha::ChaCha8Rng::from_seed(rng_seed);

        let map = Map::new(&mut rng, 23, 23);

        Ok(Game {
            rng_seed,
            rng,

            map,
            player: Player::default(),

            shows_mini_map: false,

            map_view_count: 0,
            key_press_count: 0,

            rendering_data: None,
        })
    }

    pub fn draw(&mut self, surface: &PrimarySurface) -> Result<(), Error> {
        if let None = self.rendering_data {
            self.create_rendering_data(surface)?
        }
        let r = self.rendering_data.as_mut().unwrap();

        surface.draw_rect(&rect_wh(48 - 1, 48 - 1, 256 + 2, 256 + 2), &r.black_pen);
        //self.draw_wall();
        surface.copy_from(&rect_wh(48, 48, 256, 256), &r.map_surface, 0, 0);

        Ok(())
    }

    fn create_rendering_data(&mut self, surface: &PrimarySurface) -> Result<(), Error> {
        let map_surface = surface.create_surface(256, 256);
        let wall_surface = surface.load_bitmap("assets\\wall.bmp")?;
        let mini_map_surface = surface.create_surface(256, 256);
        let black_pen = create_solid_pen(1, color_rgb(0, 0, 0));
        let black_brush = create_solid_brush(color_rgb(255, 0, 0));
        self.rendering_data = Some(RenderingData {
            map_surface,
            wall_surface,
            mini_map_surface,
            black_pen,
            black_brush,
        });
        Ok(())
    }

    fn draw_wall(&mut self) {
        let r = self.rendering_data.as_mut().unwrap();
    }
}

impl Drop for Game {
    fn drop(&mut self) {}
}
