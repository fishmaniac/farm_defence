use crate::constants;
use crate::event_manager;
use crate::player_manager;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::LevelTile;
use crate::level_manager::TileData;
use crate::texture_manager;
use crate::projectile_manager;
use crate::gui_manager;
use crate::tower_manager;
use crate::pathfinding_manager;
use crate::utilities;
use crate::utilities::check_enemy_collisions;
use crate::utilities::clamp_speed;

pub struct Enemy {
    pub final_path: Option<Vec<(usize, usize)>>,
    pub cost_total: f32,
    pub current_target: Option<(usize, usize)>,
    pub grid_index: (usize, usize),
    pub pixel_index: (u32, u32),
    pub max_health: u16,
    pub health: u16,
    pub movement_speed: u16,
    pub attack_damage: u8,
    pub attack_radius: u8,
    pub attack_speed: u8,
    pub direction: player_manager::Direction,
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
}

pub struct EnemyManager {
    pub enemy_vec: Vec<Enemy>,
}

impl EnemyManager {
    pub fn new () -> EnemyManager {
        let enemies = EnemyManager {
            enemy_vec: Vec::new(),
        };
        enemies
    }

    pub fn place_enemy(
        &mut self, 
        game: &mut game_manager::GameManager,
        temp_tile: &level_manager::LevelTile,
        tile_data: level_manager::TileData,
        index: (usize, usize),
    ) {
        match tile_data {
            TileData::Goblin => {
                let temp_enemy = self::Enemy {
                    final_path: None,
                    cost_total: 0.0,
                    movement_speed: constants::ENEMY_GOBLIN_SPEED,
                    attack_damage: constants::ENEMY_GOBLIN_DAMAGE,
                    attack_radius: constants::ENEMY_GOBLIN_RADIUS,
                    attack_speed: constants::ENEMY_GOBLIN_ATTACK_SPEED,
                    max_health: constants::ENEMY_GOBLIN_HEALTH,
                    health: constants::ENEMY_GOBLIN_HEALTH,
                    current_target: None,
                    grid_index: index,
                    pixel_index: (index.0 as u32 * constants::TILE_SIZE, index.1 as u32 * constants::TILE_SIZE),
                    direction: player_manager::Direction::Down,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string(),
                };
                self.enemy_vec.push(temp_enemy);
            },
            _=> {
                let temp_enemy = self::Enemy {
                    final_path: None,
                    cost_total: 0.0,
                    movement_speed: 1,
                    attack_damage: 1,
                    attack_radius: 1,
                    attack_speed: 1,
                    max_health: 1,
                    health: 1,
                    current_target: None,
                    grid_index: index,
                    pixel_index: (index.0 as u32 * constants::TILE_SIZE, index.1 as u32 * constants::TILE_SIZE),
                    direction: player_manager::Direction::Down,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                self.enemy_vec.push(temp_enemy);
            }
        }
    }

    pub fn render_enemies(
        &mut self,
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        gui_manager: &mut gui_manager::GUIManager,
    ) -> Result<(), String> {
        for enemy in &mut self.enemy_vec {
            enemy.rect.set_x(enemy.pixel_index.0 as i32 - game.cam_x);
            enemy.rect.set_y(enemy.pixel_index.1 as i32 - game.cam_y);

            let texture = tex_man.load(&enemy.texture_path)?;

            game.canvas.copy_ex(
                &texture,
                None,
                enemy.rect,
                0.0,
                None,
                false,
                false,
            )?;
            if enemy.health < enemy.max_health {
                gui_manager.render_health_bar_enemy(game, enemy);
            }
        }
        Ok(())
    }
    pub fn repath_all_enemies (&mut self) {
        for enemy in &mut self.enemy_vec {
            //add check if path intersects placed object
            //*called in building manager
            enemy.final_path = None;
            enemy.current_target = None;
        }
    }

    pub fn move_enemies (
        &mut self,
        events: &mut event_manager::EventManager,
        game: &mut game_manager::GameManager,
        level: &mut level_manager::LevelManager, 
        pathfinding_manager: &mut pathfinding_manager::PathfindingManager,
    ) {
        for enemy in &mut self.enemy_vec {
            let is_targets: bool = !game.target_vec.is_empty();

            if let Some(enemy_path) = enemy.final_path.take().as_mut() {
                if enemy_path.is_empty() {
                    enemy.final_path = None;
                    return;
                }
                let speed: u16 = (enemy.movement_speed as f64 * events.delta_time as f64) as u16;

                let target_pixel_index = (enemy_path[0].0 as u32 * constants::TILE_SIZE,
                    enemy_path[0].1 as u32 * constants::TILE_SIZE);
                let mut new_pixel_index: (u32, u32) = enemy.pixel_index;
                let distance_to_target = ((target_pixel_index.0 as i32 - new_pixel_index.0 as i32).abs(),
                    (target_pixel_index.1 as i32 - new_pixel_index.1 as i32).abs());

                if distance_to_target.0 <= speed as i32 
                && distance_to_target.1 <= speed as i32 {
                    enemy.pixel_index = target_pixel_index;
                    enemy.grid_index = ((enemy.pixel_index.0 / constants::TILE_SIZE) as usize,
                        (enemy.pixel_index.1 / constants::TILE_SIZE) as usize);
                    enemy_path.remove(0);
                }
                else {
                    if new_pixel_index.0 > target_pixel_index.0 {
                        new_pixel_index.0 -= speed as u32;
                    }
                    else if new_pixel_index.0 < target_pixel_index.0 {
                        new_pixel_index.0 += speed as u32;
                    }
                    if new_pixel_index.1 > target_pixel_index.1 {
                        new_pixel_index.1 -= speed as u32;
                    }
                    else if new_pixel_index.1 < target_pixel_index.1 {
                        new_pixel_index.1 += speed as u32;
                    }
                    enemy.pixel_index = new_pixel_index;
                    if enemy.pixel_index.0 % 32 == 0 
                    && enemy.pixel_index.1 % 32 == 0 {
                        enemy.grid_index = ((enemy.pixel_index.0 / 32) as usize, 
                            (enemy.pixel_index.1 / 32) as usize);
                    }
                }
                enemy.final_path = Some(enemy_path.to_vec());
            } 
            else if !game.is_pathfinding && is_targets && enemy.current_target.is_none() {
                let target = game.target_vec[game.frame_time as usize % game.target_vec.len()];
                enemy.final_path = None;
                pathfinding_manager.astar(enemy, target, &level.level_vec);
                enemy.current_target = Some(target);
                game.is_pathfinding = true;
            }
        }
    }
}
