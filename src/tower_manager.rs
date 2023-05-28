use sdl2::video::WindowContext;

use crate::constants;
use crate::level_manager;
use crate::level_manager::TileData;
use crate::game_manager;
use crate::texture_manager;
use crate::player_manager;

pub struct Tower {
    pub attack_speed: i8,
    pub attack_damage: i8,
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
}

pub struct TowerManager {
    tower_vec: Vec<Tower>,
}

impl TowerManager {
    pub fn new () -> TowerManager {
        let towers = TowerManager {
            tower_vec: Vec::new(),
        };
        towers
    }

    pub fn place_tower(&mut self, temp_tile: &level_manager::LevelTile, player: &mut player_manager::PlayerManager) {
        match temp_tile.tile_data {
            TileData::ArcherTower => {
                let tower_tile = self::Tower {
                    attack_speed: 5,
                    attack_damage: 5,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_TOWER_ARCHER_BOTTOM.to_string(),
                };
                self.tower_vec.push(tower_tile);
/*                 println!("ARCHER TOWER PUSHED"); */
            },
            _=> {
                let tower_tile = self::Tower {
                    attack_speed: 5,
                    attack_damage: 5,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                self.tower_vec.push(tower_tile);
            }
        }
    }
    
    pub fn render_towers(&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, player: &mut player_manager::PlayerManager) -> Result<(), String> {
        for (tower_index, tower) in self.tower_vec.iter_mut().enumerate() {
            tower.rect = sdl2::rect::Rect::new(tower.rect.x(), tower.rect.y(), tower.rect.width(), tower.rect.height());
            let texture = tex_man.load(&tower.texture_path)?;
            game.canvas.copy_ex(
                &texture, // Texture object
                None,      // source rect
                tower.rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;
            println!("||TOWER RECT|| X: {}, Y{}", tower.rect.x(), tower.rect.y())
        }
        Ok(())
    }
}
