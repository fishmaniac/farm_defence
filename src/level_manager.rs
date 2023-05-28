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
use crate::tower_manager;

pub enum TileData {
    Carrots,
    Tomatoes,
    ArcherTower,
    None,
}

// pub struct Tower {
//     pub row_index: i32,
//     pub col_index: i32,
//     pub attack_speed: i8,
//     pub attack_damage: i8,
//     pub rect: sdl2::rect::Rect,
//     pub texture_path: String,
// }
//
// pub struct TowerList {
//     tower_vec: Vec<Tower>,
// }

pub struct LevelManager {
    level_vec: Vec<Vec<LevelTile>>,
}

pub struct LevelTile {
    pub tile_type: char,
    pub prev_type: char,
    pub texture_path: String,
    pub rect: Rect,
    pub state: u32,
    pub tile_data: TileData,
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
                    tile_type: constants::TILE_TYPE_GRASS,
                    prev_type: constants::TILE_TYPE_GRASS,
                    texture_path: constants::TEXTURE_TILE_EMPTY.to_string(),
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
                    constants::TILE_TYPE_GRASS => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_TILE_EMPTY.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_WALL => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_TILE_WALL.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_FLOOR => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_TILE_FLOOR.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_FIELD_EMPTY => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            texture_path: constants::TEXTURE_FIELD_EMPTY.to_string(),
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

    pub fn render_level(&mut self, game: &mut GameManager, player: &mut PlayerManager, tex_man: &mut TextureManager<WindowContext>, seed_buttons: &mut ButtonManager, build_buttons: &mut ButtonManager, towers: &mut tower_manager::TowerManager) -> Result<(), String> {
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
                    if temp_tile.tile_type == constants::TILE_TYPE_WALL {
                        player.colliding = true;
                    }
                    else {
                        player.colliding = false;
                    }
                }
                Self::update_buildings(game, temp_tile, seed_buttons, build_buttons, towers, player, row_index, col_index);
                // if temp_tile.tile_data == TileData::ArcherTower {
                //     
                match temp_tile.tile_data {
                    self.level_vec.get(row_index.wrapping_sub(1));
                    TileData::ArcherTower => {
                        let rect = Rect::new(
                            (constants::TILE_SIZE as i32 * col_index as i32) - game.cam_x,
                            (constants::TILE_SIZE as i32 * row_index as i32) - game.cam_y,
                            constants::TILE_SIZE,
                            constants::TILE_SIZE,
                        );  
                        let texture = tex_man.load(constants::TEXTURE_TOWER_ARCHER_BOTTOM)?;
                        game.canvas.copy_ex(
                            &texture, // Texture object
                            None,      // source rect
                            rect,     // destination rect
                            0.0,      // angle (degrees)
                            None,   // center
                            false,    // flip horizontal
                            false,     // flip vertical
                        )?;
                    }
                    _ => {},
                    }
            }
        }
        Ok(())
    }

    fn update_buildings(game: &mut GameManager, temp_tile: &mut LevelTile, seed_buttons: &mut ButtonManager, build_buttons: &mut ButtonManager, towers: &mut tower_manager::TowerManager, player: &mut PlayerManager, row_index: usize, col_index: usize) {
        //INCREASE ALL FARM STATE
        match temp_tile.tile_data {
            TileData::Carrots | TileData::Tomatoes => {
                match temp_tile.tile_type {
                    constants::TILE_TYPE_FIELD_EMPTY | constants::TILE_TYPE_FIELD_GROWING | constants::TILE_TYPE_FIELD_HARVESTABLE => temp_tile.state += 1,
                    _ => {},
                }
            }
            _ => {}
        }
        //PRETTY SURE HOVERING ALL BUTTONS = BUG
        if/*  !seed_buttons.hovering_all_buttons && !build_buttons.hovering_all_buttons &&  */Rect::contains_point(&temp_tile.rect, game.mouse_point) && game.mouse_button == MouseButton::Left {
            if game.build_mode {
                match game.current_build {
                    //BUILD MODE HO
                    build if build == constants::CURRENT_BUILD_HO as usize => {
                        if temp_tile.prev_type == constants::TILE_TYPE_GRASS {
                            if temp_tile.tile_type == constants::TILE_TYPE_FIELD_HARVESTABLE {
                                match temp_tile.tile_data {
                                    TileData::Carrots => game.carrot_amount += 1,
                                    TileData::Tomatoes => game.tomato_amount += 1,
                                    _ => {},
                                }
                            }
                            temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                            temp_tile.texture_path = constants::TEXTURE_FIELD_EMPTY.to_string();
                            temp_tile.tile_data = TileData::None;
                            println!("CARROTS: {}, TOMATOS: {}", game.carrot_amount, game.tomato_amount);
                        }
                    }
                    //BUILD MODE ARCHER TOWER
                    build if build == constants::CURRENT_BUILD_ARCHER_TOWER as usize => {
                        towers.place_tower(&temp_tile, player, row_index, col_index);

                        temp_tile.tile_type = constants::TILE_TYPE_ARCHER_BOTTOM;
                        /*                         temp_tile.texture_path = constants::TEXTURE_TOWER_ARCHER_BOTTOM.to_string();  */
                        temp_tile.tile_data = TileData::ArcherTower;
                    }
                    _ => {}
                }
            }
            if game.seed_mode && temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY {
                match game.current_seed {
                    seed if seed == constants::CURRENT_SEED_CARROT as usize => {
                        temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                        temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                        temp_tile.tile_data = TileData::Carrots;
                    }
                    seed if seed == constants::CURRENT_SEED_TOMATO as usize => {
                        temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                        temp_tile.texture_path = constants::TEXTURE_FIELD_SEEDS.to_string();
                        temp_tile.tile_data = TileData::Tomatoes;
                    }
                    _ => {}
                }
            }
        }

        //CHANGE TO GROWING FARM STATE
        if temp_tile.tile_type == constants::TILE_TYPE_FIELD_EMPTY && temp_tile.state == constants::CROP_TIME {
            match temp_tile.tile_data {
                TileData::Carrots | TileData::Tomatoes => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_GROWING;
                    temp_tile.texture_path = constants::TEXTURE_FIELD_GROWING.to_string();
                    temp_tile.state = 0;
                }
                _ => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_EMPTY;
                    temp_tile.texture_path = constants::TEXTURE_DEFAULT.to_string();
                    temp_tile.state = 0;
                }
            }
        }

        //CHANGE TO HARVEST FARM STATE
        if temp_tile.tile_type == constants::TILE_TYPE_FIELD_GROWING && temp_tile.state == constants::CROP_TIME {
            match temp_tile.tile_data {
                TileData::Carrots => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_HARVESTABLE;
                    temp_tile.texture_path = constants::TEXTURE_FIELD_CARROT.to_string();
                    temp_tile.state = 0;
                }
                TileData::Tomatoes => {
                    temp_tile.tile_type = constants::TILE_TYPE_FIELD_HARVESTABLE;
                    temp_tile.texture_path = constants::TEXTURE_FIELD_TOMATO.to_string();
                    temp_tile.state = 0;
                }
                _ => {
                    temp_tile.tile_type = constants::TILE_TYPE_GRASS;
                    temp_tile.texture_path = constants::TEXTURE_DEFAULT.to_string();
                    temp_tile.state = 0;
                }
            }
        }
    }
}
