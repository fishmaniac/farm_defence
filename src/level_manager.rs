use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

use crate::{constants, projectile_manager, gui_manager, game_manager, building_manager, player_manager, event_manager};
use crate::texture_manager::TextureManager;
use crate::tower_manager;
use crate::enemy_manager;

#[derive(PartialEq)]
pub enum TileData {
    Base,
    ArcherTowerBottom,
    ArcherTowerTop,
    FireballTowerBottom,
    FireballTowerTop,
    Carrots,
    Tomatoes,
    Goblin,
    None,
}

pub struct LevelManager {
    pub level_vec: Vec<Vec<LevelTile>>,
}

pub struct LevelTile {
    pub tile_type: char,
    pub prev_type: char,
    pub original_type: char,
    pub texture_path: String,
    pub rect: sdl2::rect::Rect,
    pub state: u16,
    pub tile_data: TileData,
    pub is_occupied: bool,
}

impl LevelManager {
    pub fn new() -> LevelManager {
        let level = LevelManager {
            level_vec: Vec::new(),
        };
        level
    }

    // pub fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
    //
    //     Ok(())
    // }
    //
    //
    // pub fn load_from_file(file_path: &str) -> Result<LevelManager, std::io::Error> {
    //
    //     Ok(LevelManager { level_vec })
    // }

    pub fn create_level(&mut self) {
        for _ in 0..constants::MAX_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..constants::MAX_WIDTH {
                let rect = sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE);

                row.push(LevelTile { 
                    tile_type: constants::TILE_TYPE_GRASS,
                    prev_type: constants::TILE_TYPE_GRASS,
                    original_type: constants::TILE_TYPE_GRASS,
                    texture_path: constants::TEXTURE_TILE_GRASS.to_string(),
                    rect,
                    state: 0,
                    tile_data: TileData::None,
                    is_occupied: false,
                });
            }
            self.level_vec.push(row);
        }
    }

    pub fn read_file(
        &mut self, 
        filename: &str) 
    -> Result<(), std::io::Error> {
        println!("Reading from dir: {:?}", env::current_dir()?);
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut temp_vec: Vec<Vec<LevelTile>> = Vec::new();
        let rect = sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE);

        for line in reader.lines() {
            let line = line?;
            let mut row_vec: Vec<LevelTile> = Vec::new();
            for ch in line.chars() {
                match ch {
                    constants::TILE_TYPE_GRASS => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            original_type: ch,
                            texture_path: constants::TEXTURE_TILE_GRASS.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                            is_occupied: false,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_WALL => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            original_type: ch,
                            texture_path: constants::TEXTURE_TILE_WALL.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                            is_occupied: false,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_FLOOR => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            original_type: ch,
                            texture_path: constants::TEXTURE_TILE_FLOOR.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                            is_occupied: false,
                        };
                        row_vec.push(tile);
                    }
                    constants::TILE_TYPE_FIELD_EMPTY => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            original_type: ch,
                            texture_path: constants::TEXTURE_FIELD_EMPTY.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                            is_occupied: false,
                        };
                        row_vec.push(tile);
                    }
                    _ => {
                        let tile = LevelTile {
                            tile_type: ch,
                            prev_type: ch,
                            original_type: ch,
                            texture_path: constants::TEXTURE_DEFAULT.to_string(),
                            rect,
                            state: 0,
                            tile_data: TileData::None,
                            is_occupied: false,
                        };
                        row_vec.push(tile);
                    }
                }
            }
            temp_vec.push(row_vec);
        }
        self.level_vec = temp_vec;
        Ok(())
    }

    pub fn render_level(
        &mut self,
        game: &mut game_manager::GameManager, 
        tex_man: &mut TextureManager<sdl2::video::WindowContext>,
        player: &mut player_manager::PlayerManager,
        events: &mut event_manager::EventManager,
    ) -> Result<(), String> {
        for col_index in 0..self.level_vec.len() {
            for row_index in 0..self.level_vec[col_index].len() {
                let temp_tile = &mut self.level_vec[col_index][row_index];
                temp_tile.rect.set_x(constants::TILE_SIZE as i32 * col_index as i32 - game.cam_x);
                temp_tile.rect.set_y(constants::TILE_SIZE as i32 * row_index as i32 - game.cam_y);
                let screen_rect = sdl2::rect::Rect::new(player.rect.x() - (events.screen_size.0 / 2), player.rect.y() - (events.screen_size.1 / 2), events.screen_size.0 as u32, events.screen_size.1 as u32);

                if temp_tile.rect.has_intersection(screen_rect) {
                    let texture = tex_man.load(&temp_tile.texture_path)?;
                    game.canvas.copy_ex(
                        &texture,
                        None,
                        temp_tile.rect,
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                }
            }
        }
        Ok(())
    }
    pub fn render_level_minimap(
        &mut self,
        game: &mut game_manager::GameManager, 
        tex_man: &mut TextureManager<sdl2::video::WindowContext>,
        player: &mut player_manager::PlayerManager,
        events: &mut event_manager::EventManager,
    ) -> Result<(), String> {
        //SAVE BITMAP

        // for col_index in 0..self.level_vec.len() {
        //     for row_index in 0..self.level_vec[col_index].len() {
        //         let texture_path = &self.level_vec[col_index][row_index].texture_path;
        //
        //         /*                 let test = sdl2::rect::Rect::new(player.rect.x(), player.rect.y(), 100, 100); */
        //         let mut rect = self.level_vec[col_index][row_index].rect;
        //
        //         // game.canvas.set_draw_color(sdl2::pixels::Color::RED);
        //         // game.canvas.fill_rect(test);
        //
        //
        //
        //         rect.set_x(col_index as i32);
        //         rect.set_y(row_index as i32);
        //         rect.set_width(1);
        //         rect.set_height(1);
        //
        //         if self.level_vec[col_index][row_index].rect.has_intersection(player.rect) {
        //             game.canvas.set_draw_color(sdl2::pixels::Color::RED);
        //             game.canvas.fill_rect(rect);
        //         } 
        //         else {
        //             let texture = tex_man.load(&texture_path)?;
        //
        //             game.canvas.copy_ex(
        //                 &texture,
        //                 None,
        //                 rect,
        //                 0.0,
        //                 None,
        //                 false,
        //                 false,
        //             )?;
        //         }
        //     }
        // }

        Ok(())
    }

    pub fn check_attacks (
        game: &mut game_manager::GameManager,
        events: &mut event_manager::EventManager,
        player: &mut player_manager::PlayerManager,
        enemies: &mut enemy_manager::EnemyManager, 
        towers: &mut tower_manager::TowerManager,
        buildings: &mut building_manager::BuildingManager,
        projectiles: &mut projectile_manager::ProjectileManager,
        gui_manager: &mut gui_manager::GUIManager,
    ) {
        for tower in &mut towers.tower_vec {
            let tower_pos_pixel = (constants::TILE_SIZE as i32 * tower.top_index.0 as i32, constants::TILE_SIZE as i32 * tower.top_index.1 as i32);
            for enemy in &mut enemies.enemy_vec {
                /* let enemy_pos_pixel = (constants::TILE_SIZE as i32 * enemy.grid_index.0 as i32, constants::TILE_SIZE as i32 * enemy.grid_index.1 as i32); */
                let enemy_pos_pixel = (enemy.pixel_index.0 as i32, enemy.pixel_index.1 as i32);
                //need to add delta time here
                let tower_can_attack: bool = tower_manager::TowerManager::is_within_area(tower_pos_pixel, enemy_pos_pixel, tower.attack_radius) && game.frame_time % tower.attack_speed as u32 == 0;
                let enemy_can_attack: bool = tower_manager::TowerManager::is_within_area(tower_pos_pixel, enemy_pos_pixel, enemy.attack_radius as i32) && game.frame_time % enemy.attack_speed as u32 == 0;

                //TOWER ATTACK
                if enemy.health != 0 {
                    if tower_can_attack && !tower.is_attacking {
                        if tower_pos_pixel != enemy_pos_pixel {
                            projectiles.spawn_tower_projectile(tower, tower_pos_pixel, tower_pos_pixel, enemy_pos_pixel);
                            tower.is_attacking = true;
                        }
                    }
                }
                //ENEMY ATTACK
                if tower.health != 0 && enemy_can_attack {
                    if tower.health > enemy.attack_damage as u16 {
                        tower.health -= enemy.attack_damage as u16;
                    }
                    else {
                        tower.health = 0;
                    }
                    /*                     enemy.found_target = true; */
                }

            }
            tower.is_attacking = false;
        }
        for building in &mut buildings.building_vec {
            for enemy in &mut enemies.enemy_vec {
                let enemy_pos_pixel = (constants::TILE_SIZE as i32 * enemy.grid_index.0 as i32, constants::TILE_SIZE as i32 * enemy.grid_index.1 as i32);
                let enemy_can_attack: bool = tower_manager::TowerManager::is_within_area(building.pixel_index, enemy_pos_pixel, enemy.attack_radius as i32);
                if building.health != 0 && enemy_can_attack {
                    if building.health > enemy.attack_damage as u16 {
                        building.health -= enemy.attack_damage as u16;
                        building.last_damaged = 0;
                    }
                    else {
                        building.health = 0;
                    }
                    /*                     enemy.found_target = true; */
                    gui_manager.create_unique_message("base is under attack!".to_string(), 128);
                }

            }

        }
    }
}
