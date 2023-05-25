use sdl2::rect::Rect;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::game_manager::GameManager;
use crate::player_manager::PlayerManager;


const MAX_WIDTH: u32 = 300;
const MAX_HEIGHT: u32 = 30;
const TILE_SIZE: u32 = 32;

pub struct LevelManager {
    tile_type: char,
    level_vec: Vec<Vec<LevelTile>>,
}

pub struct LevelTile {
    tile_type: char,
}

impl LevelManager {
    pub fn new() -> LevelManager {
        let level = LevelManager {
            tile_type: '0',
            level_vec: Vec::new(),
        };
        level
    }

    pub fn create_level(&mut self) {
        for _ in 0..MAX_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..MAX_WIDTH {
                row.push(LevelTile { tile_type: '0'});
            }

            self.level_vec.push(row);
        }
    }

    pub fn render_level(&mut self, game: &mut GameManager, player: &mut PlayerManager) {
        let mut color:sdl2::pixels::Color = sdl2::pixels::Color::RGBA(0, 0, 0, 255);
        let mut temp_tile:LevelTile;

        for (row_index, row) in self.level_vec.iter().enumerate() {
            for (col_index, temp_tile) in row.iter().enumerate() {
                let rect = Rect::new(
                    (TILE_SIZE as i32 * col_index as i32) - game.cam_x,
                    (TILE_SIZE as i32 * row_index as i32) - game.cam_y,
                    TILE_SIZE,
                    TILE_SIZE,
                );  

                color = match temp_tile.tile_type {
                    '0' => sdl2::pixels::Color::RGBA(255, 255, 255, 255),
                    '2' => sdl2::pixels::Color::RGBA(0, 0, 255, 255),
                    '3' => sdl2::pixels::Color::RGBA(0, 255, 0, 255),
                    _ => color, // Handle other cases if needed
                };

                game.canvas.set_draw_color(color);
                game.canvas.fill_rect(rect).unwrap();
            }
        }

    }
    pub fn read_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut temp_vec: Vec<Vec<LevelTile>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut row_vec: Vec<LevelTile> = Vec::new();

            for ch in line.chars() {
                let tile = LevelTile { tile_type: ch };
                row_vec.push(tile);
            }

            temp_vec.push(row_vec);
        }

        self.level_vec = temp_vec;
        Ok(())
    }
}
