use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::mouse::MouseButton;

pub enum ButtonType {
    Seed,
    Build,
}

use crate::{player_manager, constants, texture_manager, game_manager};

pub struct Button {
    rect: Rect,
    texture_path: String,
    pub clicked: bool,
    pub hovering_button: bool,
    pub outline_visible: bool,
}


pub struct ButtonManager {
    pub button_amount: usize,
    pub button_vec: Vec<Button>,
    pub current_button_clicked_seed: usize,
    pub current_button_clicked_build: usize,
}

impl ButtonManager {
    pub fn new(button_amount: usize, button_type: ButtonType, player: &player_manager::PlayerManager) -> ButtonManager {
        let mut buttons = ButtonManager {
            button_amount,
            button_vec: Vec::with_capacity(button_amount as usize),
            current_button_clicked_seed: 0,
            current_button_clicked_build: 0,
        };
        match button_type {
            ButtonType::Seed => buttons.create_seed_buttons(&player),
            ButtonType::Build => buttons.create_build_buttons(&player),
        }
        buttons
    }
    fn create_seed_buttons (&mut self, player: &player_manager::PlayerManager) {
        for button_index in 0..self.button_amount {
            let temp_button = self::Button {
                rect: Rect::new(player.x, player.y, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: match button_index {
                    constants::CURRENT_SEED_SHOVEL => constants::TEXTURE_BUTTON_SHOVEL.to_string(),
                    constants::CURRENT_SEED_HO => constants::TEXTURE_BUTTON_HO.to_string(),
                    constants::CURRENT_SEED_CARROT => constants::TEXTURE_BUTTON_CARROT.to_string(),
                    constants::CURRENT_SEED_TOMATO => constants::TEXTURE_BUTTON_TOMATO.to_string(),
                    _ => constants::TEXTURE_DEFAULT.to_string(),
                },
                clicked: false,
                hovering_button: false,
            };
            self.button_vec.push(temp_button);
        }
    }
    fn create_build_buttons (&mut self, player: &player_manager::PlayerManager) {
        for button_index in 0..self.button_amount {
            let temp_button = self::Button {
                rect: Rect::new(player.x, player.y, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: match button_index {
                    constants::CURRENT_BUILD_ARCHER_TOWER => constants::TEXTURE_BUTTON_ARCHER.to_string(),
                    constants::CURRENT_BUILD_GOBLIN => constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string(),
                    _ => constants::TEXTURE_DEFAULT.to_string(),
                },
                clicked: false,
                hovering_button: false,
            };
            self.button_vec.push(temp_button);
        }
    }

    fn draw_rect_outline(game: &mut game_manager::GameManager, rect: Rect) {
        game.canvas.set_draw_color(constants::COLOR_OUTLINE);
        game.canvas.draw_line(rect.top_left(), rect.top_right()).unwrap();
        game.canvas.draw_line(rect.bottom_left(), rect.bottom_right()).unwrap();
        game.canvas.draw_line(rect.top_left(), rect.bottom_left()).unwrap();
        game.canvas.draw_line(rect.top_right(), rect.bottom_right()).unwrap();
    }

    pub fn update_clicked(&mut self, button_type: ButtonType) {
        for (button_index, button) in self.button_vec.iter_mut().enumerate() {
            match button_type {
                ButtonType::Seed => {
                    if button_index == self.current_button_clicked_seed {
                        button.clicked = true;
                    } else {
                        button.clicked = false;
                    }
                }
                ButtonType::Build => {
                    if button_index == self.current_button_clicked_build {
                        button.clicked = true;
                    } else {
                        button.clicked = false;
                    }
                }
            }
        }
    }    

    pub fn render_seed_buttons (&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, game: &mut game_manager::GameManager) -> Result<(), String> {
        if game.seed_mode {
            for button_index in 0..self.button_vec.len() {
                self.button_vec[button_index].rect.set_x(player.rect.x() + constants::TILE_SIZE as i32 * button_index as i32 - constants::SCREEN_WIDTH as i32 / 2 + constants::TILE_SIZE as i32);
                self.button_vec[button_index].rect.set_y(constants::TILE_SIZE as i32 + player.rect.y() - constants::SCREEN_HEIGHT as i32 / 2 + constants::TILE_SIZE as i32);

                let texture = tex_man.load(&self.button_vec[button_index].texture_path)?;
                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    self.button_vec[button_index].rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;
                if Rect::contains_point(&self.button_vec[button_index].rect, game.mouse_point) {
                    self.button_vec[button_index].hovering_button = true;
                }
                else {
                    self.button_vec[button_index].hovering_button = false;
                }

                if self.button_vec[button_index].hovering_button && game.mouse_button == MouseButton::Left {
                    self.button_vec[button_index].clicked = true;
                    self.current_button_clicked_seed = button_index;
                }
                if game.seed_outline_visible && self.button_vec[button_index].clicked {
                    game.current_seed = button_index;
                    Self::draw_rect_outline(game, self.button_vec[button_index].rect);
                }
            }
        }
        Ok(())
    }

    pub fn render_build_buttons (&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, game: &mut game_manager::GameManager) -> Result<(), String> {
        if game.build_mode {
            for button_index in 0..self.button_vec.len() {
                self.button_vec[button_index].rect.set_x(player.rect.x() + constants::TILE_SIZE as i32 * button_index as i32 - constants::SCREEN_WIDTH as i32 / 2 + constants::TILE_SIZE as i32);
                self.button_vec[button_index].rect.set_y(constants::TILE_SIZE as i32 + player.rect.y() - constants::SCREEN_HEIGHT as i32 / 2 + constants::TILE_SIZE as i32);

                let texture = tex_man.load(&self.button_vec[button_index].texture_path)?;
                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    self.button_vec[button_index].rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;
                if Rect::contains_point(&self.button_vec[button_index].rect, game.mouse_point) {
                    self.button_vec[button_index].hovering_button = true;
                }
                else {
                    self.button_vec[button_index].hovering_button = false;
                }
                if self.button_vec[button_index].hovering_button && game.mouse_button == MouseButton::Left {
                    self.button_vec[button_index].clicked = true;
                    self.current_button_clicked_build = button_index;
                }
                if game.build_outline_visible && self.button_vec[button_index].clicked {
                    game.current_build = button_index;
                    Self::draw_rect_outline(game, self.button_vec[button_index].rect);
                }
            }
        }
        Ok(())
    }
    pub fn is_clicked (&mut self) -> bool {
        for button_index in 0..self.button_vec.len() { 
            if self.button_vec[button_index].hovering_button == true {
                return true;
            }
        }
        return false;
    }

}
