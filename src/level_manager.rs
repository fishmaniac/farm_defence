use crate::game_manager::GameManager;
use sdl2::rect::Rect;

const MAX_WIDTH: u32 = 300;
const MAX_HEIGHT: u32 = 30;
const TILE_SIZE: u32 = 32;

pub struct LevelManager {
    tile_type: i32,
    level_vec: Vec<Vec<i32>>,
}

impl LevelManager {
    pub fn new() -> LevelManager {
        let level = LevelManager {
            tile_type: 0,
            level_vec: Vec::new(),
        };
        level
    }

    pub fn create_level(&mut self) {
        for _ in 0..MAX_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..MAX_WIDTH {
                row.push(0);
            }
            self.level_vec.push(row);
        }
    }

    pub fn render_level(&mut self, game: &mut GameManager) {
        let mut color;
        for (row_index, row) in self.level_vec.iter().enumerate() {
            for (col_index, &tile_type) in row.iter().enumerate() {
                let rect = Rect::new(
                    (TILE_SIZE as i32 * col_index as i32).try_into().unwrap(),
                    (TILE_SIZE as i32 * row_index as i32).try_into().unwrap(),
                    TILE_SIZE,
                    TILE_SIZE,
                );  
                if tile_type == 0 {
                    color = sdl2::pixels::Color::RGBA(255, 255, 255, 255);
                    game.canvas.set_draw_color(color);
                    game.canvas.fill_rect(rect).unwrap();
                }
            }
        }

    }

}
