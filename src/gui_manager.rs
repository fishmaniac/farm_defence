use crate::constants;
use crate::game_manager;
use crate::enemy_manager;
use crate::texture_manager;
use crate::tower_manager;

pub struct GUI {
    pub index: (usize, usize),
    pub rect: sdl2::rect::Rect,
}

pub struct PreviewGUI {
    pub index: (usize, usize),
    pub bottom_left_rect: sdl2::rect::Rect,
    pub bottom_right_rect: sdl2::rect::Rect,
    pub top_left_rect: sdl2::rect::Rect,
    pub top_right_rect: sdl2::rect::Rect,
    pub texture_path_bottom_left: String,
    pub texture_path_bottom_right: String,
    pub texture_path_top_left: String,
    pub texture_path_top_right: String,
}

pub struct GUIManager {
    pub healthbar_vec: Vec<GUI>,
    pub preview: PreviewGUI,
}

impl GUIManager {
    pub fn new () -> Self {
        let preview = PreviewGUI {
            index: (0, 0),
            bottom_left_rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            bottom_right_rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            top_left_rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            top_right_rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path_bottom_left: "".to_string(),
            texture_path_bottom_right: "".to_string(),
            texture_path_top_left: "".to_string(),
            texture_path_top_right: "".to_string(),
        };
        let gui = GUIManager {
            healthbar_vec: Vec::new(),
            preview,
        };
        gui
    }
    pub fn render_preview (&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>) -> Result<(), String> {
        if game.preview_mode && (game.build_mode || game.seed_mode) {
            if self.preview.texture_path_bottom_left.len() > 1 {
                self.preview.bottom_left_rect.set_x(self.preview.index.0 as i32 * constants::TILE_SIZE as i32 - game.cam_x);
                self.preview.bottom_left_rect.set_y(self.preview.index.1 as i32 * constants::TILE_SIZE as i32 - game.cam_y);

                let bottom_left_texture = tex_man.load(&self.preview.texture_path_bottom_left)?;

                game.canvas.copy_ex(
                    &bottom_left_texture, // Texture object
                    None,      // source rect
                    self.preview.bottom_left_rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;
            }
            if self.preview.texture_path_top_left.len() > 1 {

                self.preview.top_left_rect.set_x(self.preview.index.0 as i32 * constants::TILE_SIZE as i32 - game.cam_x);
                self.preview.top_left_rect.set_y(self.preview.index.1 as i32 * constants::TILE_SIZE as i32 - game.cam_y - constants::TILE_SIZE as i32);

                let top_left_texture = tex_man.load(&self.preview.texture_path_top_left)?;

                game.canvas.copy_ex(
                    &top_left_texture, // Texture object
                    None,      // source rect
                    self.preview.top_left_rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;
            }
            if self.preview.texture_path_bottom_right.len() > 1 {
                self.preview.bottom_right_rect.set_x(self.preview.index.0 as i32 * constants::TILE_SIZE as i32 - game.cam_x + constants::TILE_SIZE as i32);
                self.preview.bottom_right_rect.set_y(self.preview.index.1 as i32 * constants::TILE_SIZE as i32 - game.cam_y);

                let bottom_right_texture = tex_man.load(&self.preview.texture_path_bottom_right)?;

                game.canvas.copy_ex(
                    &bottom_right_texture, // Texture object
                    None,      // source rect
                    self.preview.bottom_right_rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;
            }
            if self.preview.texture_path_top_right.len() > 1 {

                self.preview.top_right_rect.set_x(self.preview.index.0 as i32 * constants::TILE_SIZE as i32 - game.cam_x + constants::TILE_SIZE as i32);
                self.preview.top_right_rect.set_y(self.preview.index.1 as i32 * constants::TILE_SIZE as i32 - game.cam_y - constants::TILE_SIZE as i32);

                let top_right_texture = tex_man.load(&self.preview.texture_path_top_right)?;

                game.canvas.copy_ex(
                    &top_right_texture, // Texture object
                    None,      // source rect
                    self.preview.top_right_rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;
            }

        }
        Ok(())
    }
    pub fn render_health_bar_enemy (&mut self, game: &mut game_manager::GameManager, enemy: &enemy_manager::Enemy) {
        //TODO: MATCH TO ENEMY TYPE
        let max_health = sdl2::rect::Rect::new(enemy.rect.x() + (enemy.rect.width() - constants::ENEMY_GOBLIN_HEALTH_BAR_WIDTH) as i32 / 2, enemy.rect.y() - constants::ENEMY_GOBLIN_HEALTH_BAR_HEIGHT as i32, constants::ENEMY_GOBLIN_HEALTH_BAR_WIDTH, constants::ENEMY_GOBLIN_HEALTH_BAR_HEIGHT);
        let health_percentage = enemy.health as f64 / constants::ENEMY_GOBLIN_HEALTH as f64;

        let current_health = self::GUI {
            index: enemy.grid_index,
            rect: sdl2::rect::Rect::new(max_health.x(), max_health.y(), (max_health.width() as f64 * health_percentage) as u32, max_health.height()),
        };
        /*  self.gui_vec.push(temp_gui); */
        game.canvas.set_draw_color(constants::COLOR_RED);
        game.canvas.fill_rect(max_health);
        game.canvas.set_draw_color(constants::COLOR_GREEN);
        game.canvas.fill_rect(current_health.rect);
    }
    pub fn render_health_bar_tower (&mut self, game: &mut game_manager::GameManager, tower: &tower_manager::Tower) {
        //TODO: MATCH TO TOWER TYPE
        let max_health = sdl2::rect::Rect::new(tower.top_rect.x() + (tower.top_rect.width() - constants::TOWER_ARCHER_HEALTH_BAR_WIDTH) as i32 / 2, tower.top_rect.y() - constants::TOWER_ARCHER_HEALTH_BAR_HEIGHT as i32, constants::TOWER_ARCHER_HEALTH_BAR_WIDTH, constants::TOWER_ARCHER_HEALTH_BAR_HEIGHT);

        let health_percentage = tower.health as f64 / constants::TOWER_ARCHER_HEALTH as f64;

        let current_health = self::GUI {
            index: (tower.top_index.0, tower.top_index.1),
            rect: sdl2::rect::Rect::new(max_health.x(), max_health.y(), (max_health.width() as f64 * health_percentage) as u32, max_health.height()),
        };
        /*  self.gui_vec.push(temp_gui); */
        game.canvas.set_draw_color(constants::COLOR_RED);
        game.canvas.fill_rect(max_health);
        game.canvas.set_draw_color(constants::COLOR_GREEN);
        game.canvas.fill_rect(current_health.rect);
    }
}
