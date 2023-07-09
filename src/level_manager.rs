use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use std::fs::File;
use std::io::{BufRead, BufReader, Write, Read};
use std::env;

use crate::{constants, projectile_manager, gui_manager, game_manager, building_manager};
use crate::game_manager::GameManager;
use crate::texture_manager::TextureManager;
use crate::player_manager::PlayerManager;
use crate::tower_manager;
use crate::enemy_manager::EnemyManager;
use crate::enemy_manager;
use crate::button_manager;

#[derive(PartialEq)]
pub enum TileData {
    Carrots,
    Tomatoes,
    ArcherTowerBottom,
    ArcherTowerTop,
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
    pub rect: Rect,
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

    pub fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(file_path)?;

        // Write the level_vec length to the file
        let level_vec_len = self.level_vec.len() as u64;
        file.write_all(&level_vec_len.to_ne_bytes())?;

        // Write each LevelTile to the file
        for row in &self.level_vec {
            for tile in row {
                // Write the Rect data to the file
                file.write_all(&tile.rect.x().to_ne_bytes())?;
                file.write_all(&tile.rect.y().to_ne_bytes())?;
                file.write_all(&tile.rect.width().to_ne_bytes())?;
                file.write_all(&tile.rect.height().to_ne_bytes())?;

                file.write_all(&[tile.tile_type as u8])?;
                file.write_all(&[tile.prev_type as u8])?;
                file.write_all(&[tile.original_type as u8])?;
                file.write_all(tile.texture_path.as_bytes())?;
                // Serialize tile_data as a u8 variant index
                file.write_all(&[match &tile.tile_data {
                    TileData::Carrots => 0,
                    TileData::Tomatoes => 1,
                    TileData::ArcherTowerBottom => 2,
                    TileData::ArcherTowerTop => 3,
                    TileData::Goblin => 4,
                    TileData::None => 5,
                }])?;
                file.write_all(&[tile.is_occupied as u8])?;
            }
        }

        Ok(())
    }


    pub fn load_from_file(file_path: &str) -> Result<LevelManager, std::io::Error> {
        let mut file = std::fs::File::open(file_path)?;

        // Read the level_vec length from the file
        let mut level_vec_len_buf = [0; 8];
        file.read_exact(&mut level_vec_len_buf)?;
        let level_vec_len = u64::from_ne_bytes(level_vec_len_buf) as usize;

        let mut level_vec: Vec<Vec<LevelTile>> = Vec::with_capacity(level_vec_len);

        // Read each LevelTile from the file
        for _ in 0..level_vec_len {
            let rect_x = read_i32(&mut file)?;
            let rect_y = read_i32(&mut file)?;
            let rect_w = read_u32(&mut file)?;
            let rect_h = read_u32(&mut file)?;

            let tile_type_buf = read_char(&mut file)?;
            let prev_type_buf = read_char(&mut file)?;
            let original_type_buf = read_char(&mut file)?;

            // Read the length of the texture path as a u32
            let texture_path_len = read_u32(&mut file)? as usize;

            // Read the texture path string
            let mut texture_path_buf = vec![0; texture_path_len];
            file.read_exact(&mut texture_path_buf)?;
            let texture_path = match String::from_utf8(texture_path_buf) {
                Ok(path) => path,
                Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err)),
            };
            let tile_data_buf = read_u8(&mut file)?;
            let is_occupied_buf = read_u8(&mut file)?;

            let rect = sdl2::rect::Rect::new(rect_x, rect_y, rect_w, rect_h);
            let tile_type = tile_type_buf as char;
            let prev_type = prev_type_buf as char;
            let original_type = original_type_buf as char;
            let tile_data = match tile_data_buf {
                0 => TileData::Carrots,
                1 => TileData::Tomatoes,
                2 => TileData::ArcherTowerBottom,
                3 => TileData::ArcherTowerTop,
                4 => TileData::Goblin,
                5 => TileData::None,
                _ => panic!("Invalid tile data"),
            };
            let is_occupied = is_occupied_buf != 0;

            let level_tile = LevelTile {
                tile_type,
                prev_type,
                original_type,
                texture_path,
                rect,
                state: 0, // Set the default value for state (if needed)
                tile_data,
                is_occupied,
            };

            // Check if the current row vector exists
            if let Some(row_vec) = level_vec.last_mut() {
                // Push the level_tile to the row vector
                row_vec.push(level_tile);
            } else {
                // Create a new row vector and push the level_tile into it
                let mut row_vec = Vec::new();
                row_vec.push(level_tile);
                // Push the row vector to the top-level level_vec
                level_vec.push(row_vec);
            }
            if file.read(&mut [0; 1]).is_ok() {
                // If there is extra data in the buffer, print a warning
                eprintln!("Warning: Extra data found in the buffer");
            }
        }

        fn read_i32(file: &mut std::fs::File) -> Result<i32, std::io::Error> {
            let mut buf = [0; 4];
            file.read_exact(&mut buf)?;
            Ok(i32::from_ne_bytes(buf))
        }

        fn read_u32(file: &mut std::fs::File) -> Result<u32, std::io::Error> {
            let mut buf = [0; 4];
            file.read_exact(&mut buf)?;
            Ok(u32::from_ne_bytes(buf))
        }

        fn read_char(file: &mut std::fs::File) -> Result<char, std::io::Error> {
            let mut buf = [0; 1];
            file.read_exact(&mut buf)?;
            Ok(buf[0] as char)
        }

        fn read_u8(file: &mut std::fs::File) -> Result<u8, std::io::Error> {
            let mut buf = [0; 1];
            file.read_exact(&mut buf)?;
            Ok(buf[0])
        }


        Ok(LevelManager { level_vec })
    }

    pub fn create_level(&mut self) {
        for _ in 0..constants::MAX_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..constants::MAX_WIDTH {
                let rect = Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE);

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
        let rect = Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE);

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
    ) -> Result<(), String> {
        for col_index in 0..self.level_vec.len() {
            for row_index in 0..self.level_vec[col_index].len() {
                let temp_tile = &mut self.level_vec[col_index][row_index];
                temp_tile.rect.set_x((constants::TILE_SIZE as i32 * col_index as i32) - game.cam_x);
                temp_tile.rect.set_y((constants::TILE_SIZE as i32 * row_index as i32) - game.cam_y);
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
            }
        }
        Ok(())
    }

    pub fn check_attacks (
        game: &mut game_manager::GameManager,
        enemies: &mut enemy_manager::EnemyManager, 
        towers: &mut tower_manager::TowerManager,
        buildings: &mut building_manager::BuildingManager,
        projectiles: &mut projectile_manager::ProjectileManager,
    ) {
        for tower in &mut towers.tower_vec {
            let tower_pos_pixel = (constants::TILE_SIZE as i32 * tower.top_index.0 as i32, constants::TILE_SIZE as i32 * tower.top_index.1 as i32);
            for enemy in &mut enemies.enemy_vec {
                let enemy_pos_pixel = (constants::TILE_SIZE as i32 * enemy.grid_index.0 as i32, constants::TILE_SIZE as i32 * enemy.grid_index.1 as i32);
                let tower_can_attack: bool = tower_manager::TowerManager::is_within_area((tower.bottom_index.0 as i32, tower.bottom_index.1 as i32), (enemy.grid_index.0 as i32, enemy.grid_index.1 as i32), tower.attack_radius) && game.frame_time % tower.attack_speed as u32 == 0;
                let enemy_can_attack: bool = tower_manager::TowerManager::is_within_area((tower.bottom_index.0 as i32, tower.bottom_index.1 as i32), (enemy.grid_index.0 as i32, enemy.grid_index.1 as i32), enemy.attack_radius as i32) && game.frame_time % enemy.attack_speed as u32 == 0;

                //TOWER ATTACK
                if enemy.health != 0 {
                    if tower_can_attack && !tower.is_attacking {
                        if tower_pos_pixel != enemy_pos_pixel {
                            projectiles.spawn_projectile(tower, tower_pos_pixel, tower_pos_pixel, enemy_pos_pixel);
                            tower.is_attacking = true;
                        }
                        for projectile in &mut projectiles.projectile_vec {
                            let projectile_hit: bool = tower_manager::TowerManager::is_within_area(projectile.position, enemy_pos_pixel, projectile.radius as i32);

                            if projectile_hit && enemy.health != 0 && !projectile.hit_target {
                                enemy.health -= projectile.damage as u16;
                                projectile.hit_target = true;
                            }
                        }
                    }
                }
                //ENEMY ATTACK
                if tower.health != 0 && enemy_can_attack {
                    tower.health -= enemy.attack_damage as u16;
                    enemy.found_target = true;
                }

            }
            tower.is_attacking = false;
        }
        for building in &mut buildings.building_vec {
            for enemy in &mut enemies.enemy_vec {
                let enemy_pos_pixel = (constants::TILE_SIZE as i32 * enemy.grid_index.0 as i32, constants::TILE_SIZE as i32 * enemy.grid_index.1 as i32);
                let enemy_can_attack: bool = tower_manager::TowerManager::is_within_area(building.pixel_index, enemy_pos_pixel, enemy.attack_radius as i32) && game.frame_time % enemy.attack_speed as u32 == 0;
                if building.health != 0 && enemy_can_attack {
                    building.health -= enemy.attack_damage as u16;
                    enemy.found_target = true;
                    println!("Building damaged\tHealth: {}", building.health);
                }

            }

        }
    }
    pub fn delete_all_dead (
        &mut self,
        game: &mut game_manager::GameManager,
        enemies: &mut enemy_manager::EnemyManager, 
        towers: &mut tower_manager::TowerManager,
        buildings: &mut building_manager::BuildingManager,
        projectiles: &mut projectile_manager::ProjectileManager,
    ) {
        for enemy_index in (0..enemies.enemy_vec.len()).rev() {
            let enemy = &mut enemies.enemy_vec[enemy_index];

            if enemy.health == 0 {
                //MAYBE REMOVE TILE TYPE
                self.level_vec[enemy.grid_index.0][enemy.grid_index.1].tile_type = self.level_vec[enemy.grid_index.0][enemy.grid_index.1].original_type; 
                self.level_vec[enemy.grid_index.0][enemy.grid_index.1].is_occupied = false;
                self.level_vec[enemy.grid_index.0][enemy.grid_index.1].tile_data = TileData::None;
                enemies.enemy_vec.remove(enemy_index);
            }
        }
        for tower_index in (0..towers.tower_vec.len()).rev() {
            let tower = &mut towers.tower_vec[tower_index];

            if tower.health == 0 {
                for target_index in (0..game.target_vec.len()).rev() {
                    let target = game.target_vec[target_index];

                    if target == tower.bottom_index {
                        game.target_vec.remove(target_index);
                    }
                }
                for enemy_index in (0..enemies.enemy_vec.len()).rev() {
                    enemies.enemy_vec[enemy_index].found_target = false;
                }
                self.level_vec[tower.bottom_index.0][tower.bottom_index.1].tile_type = self.level_vec[tower.bottom_index.0][tower.bottom_index.1].original_type;
                self.level_vec[tower.bottom_index.0][tower.bottom_index.1].tile_data = TileData::None;
                towers.tower_vec.remove(tower_index);
            }
        }
        for projectile_index in (0..projectiles.projectile_vec.len()).rev() {
            let projectile = &mut projectiles.projectile_vec[projectile_index];
            let do_despawn_projectile: bool = (projectile.hit_target && projectile.time > constants::PROJECTILE_HIT_DESPAWN_DURATION) || projectile.time > constants::PROJECTILE_DESPAWN_DURATION;

            if do_despawn_projectile {
                projectiles.projectile_vec.remove(projectile_index);
            }
        }
        for building_index in (0..buildings.building_vec.len()).rev() {
            let building = &mut buildings.building_vec[building_index];
            if building.health == 0 {
                buildings.building_vec.remove(building_index);
            }
        }
    }
}
