use crate::building_manager;
use crate::constants;
use crate::event_manager;
use crate::game_manager;
use crate::enemy_manager;
use crate::texture_manager;
use crate::tower_manager;

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

pub struct Message {
    pub index: (usize, usize),
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
    pub message_text: String,
    pub time: u16,
    pub max_time: u16,
}

pub struct HUD {
    pub index: (usize, usize),
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
}

pub struct GUI {
    pub index: (usize, usize),
    pub rect: sdl2::rect::Rect,
}

pub struct GUIManager <'a> {
    pub healthbar_vec: Vec<GUI>,
    pub inventory_vec: Vec<HUD>,
    pub message_vec: Vec<Message>,
    pub preview: PreviewGUI,
    pub font: &'a sdl2::ttf::Font<'a, 'a>,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font_path: &'a std::path::Path,
}

impl<'a> GUIManager<'a> {
    pub fn new (game: &mut game_manager::GameManager, font: &'a sdl2::ttf::Font<'a, 'a>,
) -> Self {
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
            inventory_vec: Vec::new(),
            message_vec: Vec::new(),
            preview,
            font,
            texture_creator: game.canvas.texture_creator(),
            font_path: std::path::Path::new(&constants::FONT_PATH),
        };
        gui
    }
    pub fn create_message (&mut self, message: String, max_time: u16) {
        let message = Message {
            index: (0, 0),
            rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: constants::TEXTURE_HUD_COIN.to_string(),
            message_text: message,
            time: 0,
            max_time,
        };
        self.message_vec.push(message);
    }
    pub fn create_unique_message (&mut self, unique_message: String, max_time: u16) {
        if !self.message_vec.iter().any(|message| message.message_text == unique_message) {
            let message = Message {
                index: (0, 0),
                rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: constants::TEXTURE_HUD_COIN.to_string(),
                message_text: unique_message,
                time: 0,
                max_time,
            };
            self.message_vec.push(message);
        }
    }

    pub fn create_inventory_hud (&mut self, game: &mut game_manager::GameManager) {
        let coins = HUD {
            index: (0, 0),
            rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: constants::TEXTURE_HUD_COIN.to_string(),
        };

        self.inventory_vec.push(coins);
        let tomatoes = HUD {
            index: (0, 0),
            rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: constants::TEXTURE_BUTTON_TOMATO.to_string(),

        };

        self.inventory_vec.push(tomatoes);
        let carrots = HUD {
            index: (0, 0),
            rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: constants::TEXTURE_BUTTON_CARROT.to_string(),

        };
        self.inventory_vec.push(carrots);
        let fps = HUD {
            index: (0, 0),
            rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: constants::TEXTURE_DEFAULT.to_string(),

        };
        self.inventory_vec.push(fps);
        let delta_time = HUD {
            index: (0, 0),
            rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            texture_path: constants::TEXTURE_DEFAULT.to_string(),

        };
        self.inventory_vec.push(delta_time);

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
        let health_percentage = enemy.health as f64 / enemy.max_health as f64;

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

        let health_percentage = tower.health as f64 / tower.max_health as f64;

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
    pub fn render_health_bar_buildings (&mut self, game: &mut game_manager::GameManager, building: &building_manager::Building) {
        let max_health = sdl2::rect::Rect::new(building.top_left_rect.x(), building.top_left_rect.y(), constants::BUILDING_BASE_HEALTH_BAR_WIDTH, constants::BUILDING_BASE_HEALTH_BAR_HEIGHT);

        let health_percentage = building.health as f64 / building.max_health as f64;

        let current_health = self::GUI {
            index: (building.grid_index.0 as usize, building.grid_index.1 as usize),
            rect: sdl2::rect::Rect::new(max_health.x(), max_health.y(), (max_health.width() as f64 * health_percentage) as u32, max_health.height()),
        };
        game.canvas.set_draw_color(constants::COLOR_RED);
        game.canvas.fill_rect(max_health);
        game.canvas.set_draw_color(constants::COLOR_GREEN);
        game.canvas.fill_rect(current_health.rect);
    }
    pub fn render_inventory_hud (&mut self, events: &mut event_manager::EventManager, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>) -> Result<(), String> {
        for gui_index in 0..self.inventory_vec.len() {
            let gui = &mut self.inventory_vec[gui_index];
            gui.rect.set_x(game.screen_size.0 - 4 * constants::TILE_SIZE as i32);
            gui.rect.set_y(2 * constants::TILE_SIZE as i32 + (constants::TILE_SIZE as i32 * gui_index as i32));

            let text_surface: sdl2::surface::Surface;
            match gui_index {
                0 => {
                    text_surface = self.font.render(&game.gold_amount.to_string())
                        .blended(constants::COLOR_WHITE)
                        .map_err(|e| e.to_string())?;
                },
                1 => {
                    text_surface = self.font.render(&game.tomato_amount.to_string())
                        .blended(constants::COLOR_WHITE)
                        .map_err(|e| e.to_string())?;
                },
                2 => {
                    text_surface = self.font.render(&game.carrot_amount.to_string())
                        .blended(constants::COLOR_WHITE)
                        .map_err(|e| e.to_string())?;
                },
                3 => {
                    text_surface = self.font.render(&format!("FPS: {}", game.fps).to_string())
                        .blended(constants::COLOR_WHITE)
                        .map_err(|e| e.to_string())?;
                }
                4 => {
                    text_surface = self.font.render(&format!("dt: {}", events.delta_time).to_string())
                        .blended(constants::COLOR_WHITE)
                        .map_err(|e| e.to_string())?;
                }
                _ => {
                    text_surface = self.font.render(&"ERR".to_string())
                        .blended(constants::COLOR_WHITE)
                        .map_err(|e| e.to_string())?;
                },
            }
            match gui_index {
                0 | 1 | 2 => {
                    let texture = self.texture_creator.create_texture_from_surface(&text_surface).unwrap();
                    let dest = sdl2::rect::Rect::new(game.screen_size.0 - text_surface.width() as i32 - constants::TILE_SIZE as i32, 2 * constants::TILE_SIZE as i32 + (constants::TILE_SIZE as i32 * gui_index as i32), text_surface.width(), text_surface.height());   
                    game.canvas.copy(&texture, None, Some(dest)).unwrap(); 


                    let texture = tex_man.load(&gui.texture_path)?;
                    game.canvas.copy_ex(
                        &texture, // texture object
                        None,      // source rect
                        gui.rect,     // destination rect
                        0.0,      // angle (degrees)
                        None,   // center
                        false,    // flip horizontal
                        false,     // flip vertical
                    )?;
                },
                3 | 4 => {
                    let texture = self.texture_creator.create_texture_from_surface(&text_surface).unwrap();
                    let dest = sdl2::rect::Rect::new(game.screen_size.0 - text_surface.width() as i32 - constants::TILE_SIZE as i32, 2 * constants::TILE_SIZE as i32 + (constants::TILE_SIZE as i32 * gui_index as i32), text_surface.width(), text_surface.height());   
                    game.canvas.copy(&texture, None, Some(dest)).unwrap(); 
                },
                _ => {},
            }
        }
        Ok(())
    }
    pub fn render_messages (&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>) -> Result<(), String> {
        for message_index in (0..self.message_vec.len()).rev() {
            let message = &mut self.message_vec[message_index];
            message.rect.set_x(game.screen_size.0 / 2);
            message.rect.set_y(2 * constants::TILE_SIZE as i32 + (constants::TILE_SIZE as i32 * message_index as i32));

            let text_surface = self.font.render(&message.message_text)
                .blended(constants::COLOR_WHITE)
                .map_err(|e| e.to_string())?;
            let texture = self.texture_creator.create_texture_from_surface(&text_surface).unwrap();
            let dest = sdl2::rect::Rect::new(game.screen_size.0 / 2 - text_surface.width() as i32 / 2, 2 * constants::TILE_SIZE as i32 + (constants::TILE_SIZE as i32 * message_index as i32), text_surface.width(), text_surface.height());   
            game.canvas.copy(&texture, None, Some(dest)).unwrap(); 
            if message.time < message.max_time {
                message.time += 1;
            }
            else {
                self.message_vec.remove(message_index);
            }
        }
        Ok(())
    }
}
