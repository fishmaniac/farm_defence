use crate::constants;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::TileData;
use crate::projectile_manager;
use crate::texture_manager;
use crate::enemy_manager;
use crate::gui_manager;

pub struct Tower {
    pub bottom_index: (usize, usize),
    pub top_index: (usize, usize),
    pub bottom_rect: sdl2::rect::Rect, 
    pub bottom_texture_path: String,
    pub top_rect: sdl2::rect::Rect, 
    pub top_texture_path: String,
    pub attack_radius: i32,
    pub attack_damage: u8,
    pub attack_speed: u8,
    pub max_health: u16,
    pub health: u16,
    pub is_attacking: bool,
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
        index: (usize, usize), 
    ) {
        match temp_tile.tile_data {
            TileData::ArcherTowerBottom => {
                let tower_tile = self::Tower {
                    bottom_index: (index.0, index.1),
                    bottom_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    bottom_texture_path: constants::TEXTURE_TOWER_ARCHER_BOTTOM.to_string(),
                    top_index: (index.0, index.1 - 1),
                    top_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y() - constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    top_texture_path: constants::TEXTURE_TOWER_ARCHER_FRONT.to_string(),
                    attack_radius: constants::TOWER_ARCHER_RADIUS,
                    attack_damage: constants::TOWER_ARCHER_DAMAGE,
                    attack_speed: constants::TOWER_ARCHER_ATTACK_SPEED,
                    max_health: constants::TOWER_ARCHER_HEALTH,
                    health: constants::TOWER_ARCHER_HEALTH,
                    is_attacking: false,
                };
                game.target_vec.push((tower_tile.bottom_index.0, tower_tile.bottom_index.1));
                self.tower_vec.push(tower_tile);
            },
            _=> {
                let tower_tile = self::Tower {
                    bottom_index: (index.0, index.1),
                    bottom_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    bottom_texture_path: constants::TEXTURE_DEFAULT.to_string(),
                    top_index: (index.0, index.1 - 1),
                    top_rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y() - constants::TILE_SIZE as i32, constants::TILE_SIZE, constants::TILE_SIZE),
                    top_texture_path: constants::TEXTURE_DEFAULT.to_string(),
                    attack_radius: 0,
                    attack_damage: 0,
                    attack_speed: 0,
                    max_health: 0,
                    health: 0,
                    is_attacking: false,
                };
                game.target_vec.push((tower_tile.bottom_index.0, tower_tile.bottom_index.1));
                self.tower_vec.push(tower_tile);
            }
        }
    }
    pub fn render_towers(&mut self, 
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        health_bars: &mut gui_manager::GUIManager,
    ) -> Result<(), String> {
        for tower_bottom in &mut self.tower_vec {
            let pixel_index: (i32, i32) = (tower_bottom.bottom_index.0 as i32 * constants::TILE_SIZE as i32, tower_bottom.bottom_index.1 as i32 * constants::TILE_SIZE as i32);

            tower_bottom.bottom_rect.set_x(pixel_index.0 - game.cam_x);
            tower_bottom.bottom_rect.set_y(pixel_index.1 - game.cam_y);

            let bottom_texture = tex_man.load(&tower_bottom.bottom_texture_path)?;
            game.canvas.copy_ex(
                &bottom_texture, // Texture object
                None,      // source rect
                tower_bottom.bottom_rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;

        }
        for tower_top in &mut self.tower_vec {
            let pixel_index: (i32, i32) = (tower_top.top_index.0 as i32 * constants::TILE_SIZE as i32, tower_top.top_index.1 as i32 * constants::TILE_SIZE as i32);

            tower_top.top_rect.set_x(pixel_index.0 - game.cam_x);
            tower_top.top_rect.set_y(pixel_index.1 - game.cam_y);

            let top_texture = tex_man.load(&tower_top.top_texture_path)?;
            game.canvas.copy_ex(
                &top_texture, // Texture object
                None,      // source rect
                tower_top.top_rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;
            if tower_top.health < tower_top.max_health {
                health_bars.render_health_bar_tower(game, tower_top);
            }
        }
        Ok(())
    }

    pub fn is_within_area(tower_pos: (i32, i32), enemy_pos: (i32, i32), area: i32) -> bool {
        let dx = (tower_pos.0 - enemy_pos.0).abs();
        let dy = (tower_pos.1 - enemy_pos.1).abs();
        dx <= area && dy <= area
    }

}
