use sdl2::video::WindowContext;

use crate::constants;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::TileData;
use crate::projectile_manager;
use crate::texture_manager;
use crate::enemy_manager;
use crate::gui_manager;

pub struct Tower {
    pub bottom_row_index: usize,
    pub bottom_col_index: usize,
    pub top_row_index: usize,
    pub top_col_index: usize,
    pub bottom_rect: sdl2::rect::Rect, 
    pub bottom_texture_path: String,
    pub top_rect: sdl2::rect::Rect, 
    pub top_texture_path: String,
    pub attack_radius: i32,
    pub attack_damage: u8,
    pub attack_speed: u8,
    pub max_health: u16,
    pub health: u16,
}

pub struct TowerManager {
    pub tower_vec: Vec<Tower>,
}

impl TowerManager {
    pub fn new () -> TowerManager {
        let towers = TowerManager {
            tower_vec: Vec::new(),
        };
        towers
    }

    pub fn place_tower(
        &mut self, 
        game: &mut game_manager::GameManager,
        temp_tile: &level_manager::LevelTile, 
        col_index: usize,
        row_index: usize, 
    ) {
        println!("PLACING TOWER~ X: {}, Y: {}", row_index, col_index);
        match temp_tile.tile_data {
            TileData::ArcherTowerBottom => {
                let tower_tile = self::Tower {
                    bottom_row_index: row_index,
                    bottom_col_index: col_index,
                    bottom_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    bottom_texture_path: constants::TEXTURE_TOWER_ARCHER_BOTTOM.to_string(),
                    top_row_index: row_index - 1,
                    top_col_index: col_index,
                    top_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y() - constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    top_texture_path: constants::TEXTURE_TOWER_ARCHER_FRONT.to_string(),
                    attack_radius: constants::TOWER_ARCHER_RADIUS,
                    attack_damage: constants::TOWER_ARCHER_DAMAGE,
                    attack_speed: constants::TOWER_ARCHER_ATTACK_SPEED,
                    max_health: constants::TOWER_ARCHER_HEALTH,
                    health: constants::TOWER_ARCHER_HEALTH,
                };
                game.target_vec.push((tower_tile.bottom_col_index, tower_tile.bottom_row_index));
                self.tower_vec.push(tower_tile);
            },
            _=> {
                let tower_tile = self::Tower {
                    bottom_row_index: row_index,
                    bottom_col_index: col_index,
                    bottom_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    bottom_texture_path: constants::TEXTURE_DEFAULT.to_string(),
                    top_row_index: row_index - 1,
                    top_col_index: col_index,
                    top_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y() - constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    top_texture_path: constants::TEXTURE_DEFAULT.to_string(),
                    attack_radius: 0,
                    attack_damage: 0,
                    attack_speed: 0,
                    max_health: 0,
                    health: 0,
                };
                game.target_vec.push((tower_tile.bottom_col_index, tower_tile.bottom_row_index));
                self.tower_vec.push(tower_tile);
            }
        }
    }

    pub fn render_towers(&mut self, 
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        enemies: &mut enemy_manager::EnemyManager,
        health_bars: &mut gui_manager::GUIManager,
    ) -> Result<(), String> {

        //render bottom of tower
        for tower_index in 0..self.tower_vec.len() {
            let bottom_col = self.tower_vec[tower_index].bottom_col_index as i32;
            let bottom_row = self.tower_vec[tower_index].bottom_row_index as i32;
            self.tower_vec[tower_index].bottom_rect.set_x((constants::TILE_SIZE as i32 * bottom_col as i32) - game.cam_x);
            self.tower_vec[tower_index].bottom_rect.set_y((constants::TILE_SIZE as i32 * bottom_row as i32) - game.cam_y);

            let bottom_texture = tex_man.load(&self.tower_vec[tower_index].bottom_texture_path)?;
            game.canvas.copy_ex(
                &bottom_texture, // Texture object
                None,      // source rect
                self.tower_vec[tower_index].bottom_rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;
        }

        //render top of tower
        for tower_index in 0..self.tower_vec.len() {
            let top_col = self.tower_vec[tower_index].top_col_index as i32;
            let top_row = self.tower_vec[tower_index].top_row_index as i32;

            self.tower_vec[tower_index].top_rect.set_x((constants::TILE_SIZE as i32 * top_col as i32) - game.cam_x);
            self.tower_vec[tower_index].top_rect.set_y((constants::TILE_SIZE as i32 * top_row as i32) - game.cam_y);
            let top_texture = tex_man.load(&self.tower_vec[tower_index].top_texture_path)?;
            game.canvas.copy_ex(
                &top_texture, // Texture object
                None,      // source rect
                self.tower_vec[tower_index].top_rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;

        }
        Ok(())
    }
    //TODO: prob move to game or event man
    pub fn check_attacks(&mut self, 
        game: &mut game_manager::GameManager, 
        level: &mut level_manager::LevelManager, 
        enemies: &mut enemy_manager::EnemyManager,
        projectile_manager: &mut projectile_manager::ProjectileManager,
        health_bars: &mut gui_manager::GUIManager,
    ) {
        let mut tower_index = 0;

        while tower_index < self.tower_vec.len() {
            let tower_pos_vec = (self.tower_vec[tower_index].bottom_col_index as i32, self.tower_vec[tower_index].bottom_row_index as i32);
            let tower_pos_pixel = (constants::TILE_SIZE as i32 * self.tower_vec[tower_index].top_col_index as i32, constants::TILE_SIZE as i32 * self.tower_vec[tower_index].top_row_index as i32);

            let mut enemy_index = 0;
            while enemy_index < enemies.enemy_vec.len() {
                let enemy_pos_vec = (enemies.enemy_vec[enemy_index].col_index as i32, enemies.enemy_vec[enemy_index].row_index as i32);
                let enemy_pos_pixel = (constants::TILE_SIZE as i32 * enemy_pos_vec.0, constants::TILE_SIZE as i32 * enemy_pos_vec.1);

                //TOWER ATTACK
                if Self::is_within_radius(tower_pos_vec, enemy_pos_vec, self.tower_vec[tower_index].attack_radius) && game.frame_time % self.tower_vec[tower_index].attack_speed as u32 == 0 {
                    if enemies.enemy_vec[enemy_index].health != 0 {
                        //TODO: ADD CHECK HERE IN GAME TIME
                        //DRAW DEBUG LINE
                        game.canvas.draw_line((enemies.enemy_vec[enemy_index].rect.x(), enemies.enemy_vec[enemy_index].rect.y()), (self.tower_vec[tower_index].top_rect.x() + constants::TILE_SIZE as i32 / 2, self.tower_vec[tower_index].top_rect.y() + constants::TILE_SIZE as i32 / 2));
                        if tower_pos_pixel != enemy_pos_pixel {
                            projectile_manager.spawn_projectile(tower_pos_pixel, enemy_pos_pixel);
                        }

                        enemies.enemy_vec[enemy_index].health -= self.tower_vec[tower_index].attack_damage as u16;
                        /*                         enemies.enemy_vec[enemy_index].found_target = true; */
                    } else {
                        enemies.enemy_vec[enemy_index].found_target = false; 
                        level.level_vec[enemies.enemy_vec[enemy_index].col_index][enemies.enemy_vec[enemy_index].row_index].tile_type = level.level_vec[enemies.enemy_vec[enemy_index].col_index][enemies.enemy_vec[enemy_index].row_index].prev_type;
                        level.level_vec[enemies.enemy_vec[enemy_index].col_index][enemies.enemy_vec[enemy_index].row_index].tile_data = TileData::None;
                        enemies.enemy_vec.remove(enemy_index);
                        continue;                     
                    }
                }
                //ENEMY ATTACK
                if Self::is_within_radius(tower_pos_vec, enemy_pos_vec, enemies.enemy_vec[enemy_index].attack_radius) && game.frame_time % self.tower_vec[tower_index].attack_speed as u32 == 0 {
                    if self.tower_vec[tower_index].health != 0/*  && game.frame_time % self.tower_vec[tower_index].attack_speed as u32 == 0  */{
                        self.tower_vec[tower_index].health -= enemies.enemy_vec[enemy_index].attack_damage as u16;
                    } else {

                        let mut target_index = 0;
                        while target_index < game.target_vec.len() {
                            if Self::is_within_radius((game.target_vec[target_index].0 as i32, game.target_vec[target_index].1 as i32), enemy_pos_vec, 3) {
                                game.target_vec.remove(target_index);
                            }
                            target_index += 1;
                        }

                        enemies.enemy_vec[enemy_index].found_target = false; 
                        level.level_vec[enemies.enemy_vec[enemy_index].col_index][enemies.enemy_vec[enemy_index].row_index].tile_type = level.level_vec[enemies.enemy_vec[enemy_index].col_index][enemies.enemy_vec[enemy_index].row_index].prev_type;
                        level.level_vec[enemies.enemy_vec[enemy_index].col_index][enemies.enemy_vec[enemy_index].row_index].tile_data = TileData::None;

                        //FIXME: might need to iterate through and set all enemies within radius found_target to false
                        println!("REMOVING TOWER");
                        self.tower_vec.remove(tower_index);
                        break;
                    }
                }
                if enemy_index < enemies.enemy_vec.len() && enemies.enemy_vec[enemy_index].health < enemies.enemy_vec[enemy_index].max_health {
                    health_bars.render_health_bar_enemy(game, &enemies.enemy_vec[enemy_index]);
                }
                enemy_index += 1;
            }
            if tower_index < self.tower_vec.len() && self.tower_vec[tower_index].health < self.tower_vec[tower_index].max_health {
                health_bars.render_health_bar_tower(game, &self.tower_vec[tower_index]);
            }
            tower_index += 1;
        }
    }
    pub fn is_within_radius(tower_pos: (i32, i32), enemy_pos: (i32, i32), radius: i32) -> bool {
        let dx = (tower_pos.0 - enemy_pos.0).abs();
        let dy = (tower_pos.1 - enemy_pos.1).abs();
        dx <= radius && dy <= radius
    }
}
