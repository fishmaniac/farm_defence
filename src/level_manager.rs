use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

use crate::constants;
use crate::game_manager::GameManager;
use crate::player_manager::PlayerManager;
use crate::texture_manager::TextureManager;
use crate::button_manager::ButtonManager;

pub enum TileData {
    Carrots,
    Tomatoes,
    ArcherTower,
    None,
}

pub struct LevelManager {
    level_vec: Vec<Vec<LevelTile>>,
}

pub struct LevelTile {
    tile_type: char,
    texture_path: String,
    pub rect: Rect,
    state: u32,
    tile_data: TileData,
}

impl LevelManager {
    pub fn new() -> LevelManager {
        let level = LevelManager {
            level_vec: Vec::new(),
        };
        level
    }

    pub fn create_level(&mut self) {
        for _ in 0..constants::MAX_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..constants::MAX_WIDTH {
                let rect = Rect::new(0, 0, 0, 0);

                row.push(LevelTile { 
                    tile_type: '0',
                    texture_path: "assets/tile1.png".to_string(),
                    rect,
                    state: 0,
                    tile_data: TileData::None,
                });
            }
            self.level_vec.push(row);
        }
    }
    
    pub fn read_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        println!("Reading from dir: {:?}", env::current_dir()?);
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut temp_vec: Vec<Vec<LevelTile>> = Vec::new();
        let rect = Rect::new(0, 0, 0, 0);

        for line in reader.lines() {
            let line = line?;
            let mut row_vec: Vec<LevelTile> = Vec::new();
            for ch in line.chars() {
                match ch {
                    '0' => {
                        let tile = LevelTile {
                            tile_type: ch,
                            texture_path: "assets/tile1.png".to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    '2' => {
                        let tile = LevelTile {
                            tile_type: ch,
                            texture_path: "assets/tile2.png".to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    '3' => {
                        let tile = LevelTile {
                            tile_type: ch,
                            texture_path: "assets/tile3.png".to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    'F' => {
                        let tile = LevelTile {
                            tile_type: ch,
                            texture_path: "assets/field0.png".to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
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

    pub fn render_level(&mut self, game: &mut GameManager, player: &mut PlayerManager, tex_man: &mut TextureManager<WindowContext>, seed_buttons: &mut ButtonManager, build_buttons: &mut ButtonManager) -> Result<(), String> {
        for (row_index, row) in self.level_vec.iter_mut().enumerate() {
            for (col_index, mut temp_tile) in row.iter_mut().enumerate() {
                temp_tile.rect = Rect::new(
                    (constants::TILE_SIZE as i32 * col_index as i32) - game.cam_x,
                    (constants::TILE_SIZE as i32 * row_index as i32) - game.cam_y,
                    constants::TILE_SIZE,
                    constants::TILE_SIZE,
                );  
                let texture = tex_man.load(&temp_tile.texture_path)?;
                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    temp_tile.rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;

                if Rect::has_intersection(&player.rect, temp_tile.rect){
                    if temp_tile.tile_type == '2' {
                        player.colliding = true;
                    }
                    else {
                        player.colliding = false;
                    }
                }

                Self::update_farms(game, temp_tile, seed_buttons, build_buttons);
            }
        }
        Ok(())
    }

    fn update_farms(game: &mut GameManager, temp_tile: &mut LevelTile, seed_buttons: &mut ButtonManager,
    build_buttons: &mut ButtonManager) {
        match temp_tile.tile_type {
            'F' | 'G' | 'H' => temp_tile.state += 1,
            _ => {},
        }
        //THIS IF COULD LEAD TO A BUG, BUT SEEMS OKAY
        if !seed_buttons.hovering_all_buttons && !build_buttons.hovering_all_buttons {
            match game.current_build {
                0 => {
                    match game.current_crop {
                        0 => {
                            if game.build_mode && game.current_crop == 0 && game.mouse_button == MouseButton::Left && Rect::contains_point(&temp_tile.rect, game.mouse_point) {
                                temp_tile.tile_type = 'F';
                                temp_tile.texture_path = "assets/field0.png".to_string();
                                temp_tile.tile_data = TileData::Carrots;
                            }

                        }
                        1 => {
                            if game.build_mode && game.current_crop == 1 && game.mouse_button == MouseButton::Left && Rect::contains_point(&temp_tile.rect, game.mouse_point) {
                                temp_tile.tile_type = 'F';
                                temp_tile.texture_path = "assets/field0.png".to_string();
                                temp_tile.tile_data = TileData::Tomatoes;
                            }
                        }
                        _ => {}
                    }

                }
                1 => {
                    //this will be a ho or something
                }
                _ => {}
            }
        }
        if temp_tile.tile_type == 'F' && temp_tile.state == constants::CROP_TIME {
            temp_tile.tile_type = 'G';
            temp_tile.texture_path = "assets/field1.png".to_string();
            temp_tile.state = 0;
        }
        match temp_tile.tile_data {
            TileData::Carrots => {
                if temp_tile.tile_type == 'G' && temp_tile.state == constants::CROP_TIME {
                    temp_tile.tile_type = 'H';
                    temp_tile.texture_path = "assets/carrots0.png".to_string();
                    temp_tile.state = 0;
                }
            }
            TileData::Tomatoes => {
                if temp_tile.tile_type == 'G' && temp_tile.state == constants::CROP_TIME {
                    temp_tile.tile_type = 'H';
                    temp_tile.texture_path = "assets/tomatoes0.png".to_string();
                    temp_tile.state = 0;
                }
            }
            _ => {
                if temp_tile.tile_type == 'G' && temp_tile.state == constants::CROP_TIME {
                    temp_tile.tile_type = 'H';
                    temp_tile.texture_path = "assets/carrots0.png".to_string();
                    temp_tile.state = 0;
                }
            }
        }
    }
}
