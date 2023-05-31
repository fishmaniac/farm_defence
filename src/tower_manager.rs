use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use crate::constants;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::TileData;
use crate::texture_manager;
use crate::enemy_manager;

pub struct Tower {
    pub bottom_row_index: usize,
    pub bottom_col_index: usize,
    pub top_row_index: usize,
    pub top_col_index: usize,
    pub bottom_rect: sdl2::rect::Rect, 
    pub bottom_texture_path: String,
    pub top_rect: sdl2::rect::Rect, 
    pub top_texture_path: String,
    pub attack_speed: i8,
    pub attack_damage: i8,

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
                    attack_speed: 5,
                    attack_damage: 5,
                };
                self.tower_vec.push(tower_tile);
            },
            _=> {
                let tower_tile = self::Tower {
                    bottom_row_index: row_index,
                    bottom_col_index: col_index,
                    bottom_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    bottom_texture_path: constants::TEXTURE_TOWER_ARCHER_FRONT.to_string(),
                    top_row_index: row_index - 1,
                    top_col_index: col_index,
                    top_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y() - constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    top_texture_path: constants::TEXTURE_TOWER_ARCHER_BOTTOM.to_string(),
                    attack_speed: 5,
                    attack_damage: 5,
                };
                self.tower_vec.push(tower_tile);
            }
        }
    }

    pub fn render_towers(&mut self, 
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<WindowContext>, 
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
        // PREVENT FROM PLACING BELOW TOWER
        // level.level_vec[col_index][row_index + 1].prev_type = constants::TILE_TYPE_ARCHER_BOTTOM;
        // //PREVENT FROM PLACING ON TOP OF TOWER
        // level.level_vec[col_index][row_index - 1].prev_type = constants::TILE_TYPE_ARCHER_BOTTOM;
        // //PREVENT FORM PLACING ON THIS TOWER
        // level.level_vec[col_index][row_index].prev_type = constants::TILE_TYPE_ARCHER_BOTTOM;
        // //CREATE TOP OF TOWER
        // level.level_vec[col_index][row_index - 1].tile_type = constants::TILE_TYPE_ARCHER_TOP;
        // level.level_vec[col_index][row_index - 1].tile_data = TileData::ArcherTowerTop;     
        Ok(())
    }
}
