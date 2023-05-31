use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use crate::constants;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::TileData;
use crate::texture_manager;
use crate::enemy_manager;

pub struct Tower {
    pub row_index: usize,
    pub col_index: usize,
    pub attack_speed: i8,
    pub attack_damage: i8,
    pub rect: sdl2::rect::Rect, 
    pub texture_path: String,
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

    pub fn place_tower(&mut self, 
        temp_tile: &level_manager::LevelTile, 
        row_index: usize, 
        col_index: usize) {
        println!("PLACING TOWER~ X: {}, Y: {}", row_index, col_index);
        match temp_tile.tile_data {
            TileData::ArcherTowerBottom => {
                let tower_tile = self::Tower {
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_TOWER_ARCHER_BOTTOM.to_string(),
                };
                self.tower_vec.push(tower_tile);
            },
            _=> {
                let tower_tile = self::Tower {
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                self.tower_vec.push(tower_tile);
            }
        }
    }

    pub fn render_towers(&mut self, 
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<WindowContext>, 
        level: &mut level_manager::LevelManager,
        enemies: &mut enemy_manager::EnemyManager, 
    ) -> Result<(), String> {
        for col_index in 0..level.level_vec.len() {
            for row_index in 0..level.level_vec[col_index].len() {
                let temp_tile = &mut level.level_vec[col_index][row_index];
                match temp_tile.tile_data {
                    TileData::ArcherTowerBottom => {
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
                        // PREVENT FROM PLACING BELOW TOWER
                        level.level_vec[col_index][row_index + 1].prev_type = constants::TILE_TYPE_ARCHER_BOTTOM;
                        //PREVENT FROM PLACING ON TOP OF TOWER
                        level.level_vec[col_index][row_index - 1].prev_type = constants::TILE_TYPE_ARCHER_BOTTOM;
                        //PREVENT FORM PLACING ON THIS TOWER
                        level.level_vec[col_index][row_index].prev_type = constants::TILE_TYPE_ARCHER_BOTTOM;
                        //CREATE TOP OF TOWER
                        level.level_vec[col_index][row_index - 1].tile_type = constants::TILE_TYPE_ARCHER_TOP;
                        level.level_vec[col_index][row_index - 1].tile_data = TileData::ArcherTowerTop;     
                    }
                    TileData::ArcherTowerTop => {
/*                         println!("archer tower top"); */
                        let rect = Rect::new(
                            (constants::TILE_SIZE as i32 * col_index as i32) - game.cam_x,
                            (constants::TILE_SIZE as i32 * row_index as i32) - game.cam_y,
                            constants::TILE_SIZE,
                            constants::TILE_SIZE,
                        );
                        let texture = tex_man.load(constants::TEXTURE_TOWER_ARCHER_FRONT)?;
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
                    TileData::Goblin =>  {
                        let rect = Rect::new(
                            (constants::TILE_SIZE as i32 * col_index as i32) - game.cam_x,
                            (constants::TILE_SIZE as i32 * row_index as i32) - game.cam_y,
                            constants::TILE_SIZE,
                            constants::TILE_SIZE,
                        );
                        let texture = tex_man.load(constants::TEXTURE_GOBLIN_ENEMY_FRONT)?;
                        game.canvas.copy_ex(
                            &texture, // Texture object
                            None,      // source rect
                            rect,     // destination rect
                            0.0,      // angle (degrees)
                            None,   // center
                            false,    // flip horizontal
                            false,     // flip vertical
                        )?;
                        
                        if (col_index, row_index) != (10, 30) {
                            println!("PATH: {:?}", enemies.astar((col_index, row_index), (10, 30), &mut level.level_vec)); 
/*                             level.level_vec[col_index][row_index].tile_data = TileData::None;    */

                            /*  enemies.bfs(&mut self.level_vec, (col_index, row_index), (10, 30), 0); */
                            /* level.level_vec[col_index][row_index].tile_data = TileData::None; */
                        }
                    }
                    _ => {},
                }
            }
        }
        Ok(())
    }
}
