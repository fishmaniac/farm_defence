use crate::constants;
use crate::level_manager;
use crate::level_manager::TileData;
use crate::game_manager;
use crate::texture_manager;
use crate::player_manager;

pub struct Enemy {
    pub row_index: usize,
    pub col_index: usize,
    pub attack_speed: i8,
    pub attack_damage: i8,
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

    pub fn place_enemy(&mut self, temp_tile: &level_manager::LevelTile, player: &mut player_manager::PlayerManager, row_index: usize, col_index: usize) {
        println!("PLACING ENEMY~ X: {}, Y: {}", row_index, col_index);
        match temp_tile.tile_data {
            TileData::Goblin => {
                let enemy_tile = self::Enemy {
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_TOWER_ARCHER_BOTTOM.to_string(),
                    //CHANGE TEXTURE
                };
                self.enemy_vec.push(enemy_tile);
/*                 println!("GOBLIN PUSHED"); */
            },
            _=> {
                let enemy_tile = self::Enemy {
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                self.enemy_vec.push(enemy_tile);
            }
        }
    }
}
