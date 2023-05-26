use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::video::WindowContext;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::rc::Rc;


use crate::game_manager::GameManager;
use crate::player_manager::PlayerManager;
use crate::texture_manager::TextureManager;


const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;

const IMAGE_WIDTH:u32 = 32;
const IMAGE_HEIGHT:u32 = 32;

const OUTPUT_WIDTH: u32 = IMAGE_WIDTH;
const OUTPUT_HEIGHT: u32 = IMAGE_HEIGHT;

const PLAYER_VELOCITY: i32 = 10;
const PLAYER_MAX_VELOCITY: i32 = 40;

const TILE_SIZE: u32 = 32;
const MAX_HEIGHT: u32 = 30;
const MAX_WIDTH: u32 = 300;




pub struct LevelManager {
    tile_type: char,
    level_vec: Vec<Vec<LevelTile>>,
}

pub struct LevelTile {
    tile_type: char,
    texture_path: String,
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
                row.push(LevelTile { 
                    tile_type: '0',
                    texture_path: "assets/tile1.png".to_string(),
                });
            }
            self.level_vec.push(row);
        }
    }

    pub fn render_level(&mut self, game: &mut GameManager, player: &mut PlayerManager, tex_man: &mut TextureManager<WindowContext>) -> Result<(), String> {
        let mut color:sdl2::pixels::Color = sdl2::pixels::Color::RGBA(0, 0, 0, 255);
        let mut temp_tile:LevelTile;

        for (row_index, row) in self.level_vec.iter().enumerate() {
            for (col_index, temp_tile) in row.iter().enumerate() {
                

                let src = Rect::new(
                    (TILE_SIZE as i32 * col_index as i32) - game.cam_x,
                    (TILE_SIZE as i32 * row_index as i32) - game.cam_y,
                    TILE_SIZE,
                    TILE_SIZE,
                );  

                let texture = tex_man.load(&temp_tile.texture_path)?;
                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    src,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;

            }
        }
        Ok(())
    }
    pub fn read_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        println!("Reading from dir: {:?}", env::current_dir()?);
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut temp_vec: Vec<Vec<LevelTile>> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut row_vec: Vec<LevelTile> = Vec::new();

            for ch in line.chars() {
                match ch {
                    '0' => {
                        let tile = LevelTile {
                            tile_type: ch,
                            texture_path: "assets/tile1.png".to_string(),
                        };
                        row_vec.push(tile);
                    }
                    '2' => {
                        let tile = LevelTile {
                            tile_type: ch,
                            texture_path: "assets/tile2.png".to_string(),
                        };
                        row_vec.push(tile);
                    }
                    '3' => {
                        let tile = LevelTile {
                            tile_type: ch,
                            texture_path: "assets/tile3.png".to_string(),
                        };
                        row_vec.push(tile);
                    }
                    _ => {} // Handle other cases if needed
                }

            }

            temp_vec.push(row_vec);
        }

        self.level_vec = temp_vec;
        Ok(())
    }
}
