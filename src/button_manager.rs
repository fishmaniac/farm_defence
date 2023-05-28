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
    clicked: bool,
    hovering_button: bool,
}


pub struct ButtonManager {
    pub button_amount: u8,
    pub button_vec: Vec<Button>,
    pub current_button_clicked_seed: usize,
    pub current_button_clicked_build: usize,
    pub hovering_all_buttons: bool,
}

impl ButtonManager {
    pub fn new(button_amount: u8, button_type: ButtonType, player: &player_manager::PlayerManager) -> ButtonManager {
        let mut buttons = ButtonManager {
            button_amount,
            button_vec: Vec::new(),
            current_button_clicked_seed: 0,
            current_button_clicked_build: 0,
            hovering_all_buttons: false,
        };
        match button_type {
            ButtonType::Seed => buttons.create_seed_buttons(&player),
            ButtonType::Build => buttons.create_build_buttons(&player),
        }
        buttons
    }
    fn create_seed_buttons (&mut self, player: &player_manager::PlayerManager) {
        for i in 0..self.button_amount {
            let temp_button = self::Button {
                rect: Rect::new(player.x, player.y, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: match i {
                    constants::CURRENT_SEED_CARROT => constants::TEXTURE_BUTTON_CARROT.to_string(),
                    constants::CURRENT_SEED_TOMATO => constants::TEXTURE_BUTTON_TOMATO.to_string(),
                    _ => constants::TEXTURE_DEFAULT.to_string(),
                },
                clicked: true,
                hovering_button: true,
            };
            self.button_vec.push(temp_button);
        }
    }
    fn create_build_buttons (&mut self, player: &player_manager::PlayerManager) {
        for i in 0..self.button_amount {
            let temp_button = self::Button {
                rect: Rect::new(player.x, player.y, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: match i {
                    constants::CURRENT_BUILD_HO => constants::TEXTURE_BUTTON_HO.to_string(),
                    constants::CURRENT_BUILD_FIELD => constants::TEXTURE_FIELD_EMPTY.to_string(),
                    _ => constants::TEXTURE_DEFAULT.to_string(),
                },
                clicked: true,
                hovering_button: true,
            };
            self.button_vec.push(temp_button);
        }
    }

    fn draw_rect_outline(game: &mut game_manager::GameManager, rect: Rect) {
        let color: sdl2::pixels::Color = Color::RGBA(66, 81, 245, 255);
        game.canvas.set_draw_color(color);

        game.canvas.draw_line(rect.top_left(), rect.top_right()).unwrap();
        game.canvas.draw_line(rect.bottom_left(), rect.bottom_right()).unwrap();
        game.canvas.draw_line(rect.top_left(), rect.bottom_left()).unwrap();
        game.canvas.draw_line(rect.top_right(), rect.bottom_right()).unwrap();
    }

    pub fn check_for_clicked(&mut self, button_type: ButtonType) {
        for (button_index, button) in self.button_vec.iter_mut().enumerate() {
            match button_type {
                ButtonType::Seed => {
                    if button.clicked && button_index != self.current_button_clicked_seed {
                        button.clicked = false;
                    }
                }
                ButtonType::Build => {
                    if button.clicked && button_index != self.current_button_clicked_build {
                        button.clicked = false;
                    }
                }
            }
        }
    }    

    pub fn render_seed_buttons (&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, game: &mut game_manager::GameManager) -> Result<(), String> {
        if game.seed_mode {
            for (_, temp_button) in self.button_vec.iter_mut().enumerate() {
                if Rect::contains_point(&temp_button.rect, game.mouse_point) {
                    self.hovering_all_buttons = true;
                    break
                }
                else {
                    self.hovering_all_buttons = false;
                }
            }

            for (button_index, mut temp_button) in self.button_vec.iter_mut().enumerate() {
                temp_button.rect = Rect::new(
                    player.rect.x() + constants::TILE_SIZE as i32 * button_index as i32 - constants::SCREEN_WIDTH / 2 + constants::TILE_SIZE as i32,
                    constants::TILE_SIZE as i32 + player.rect.y() - constants::SCREEN_HEIGHT / 2 + constants::TILE_SIZE as i32,
                    constants::TILE_SIZE,
                    constants::TILE_SIZE,
                );
                let texture = tex_man.load(&temp_button.texture_path)?;
                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    temp_button.rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;

                if Rect::contains_point(&temp_button.rect, game.mouse_point) {
                    temp_button.hovering_button = true;
                }
                else {
                    temp_button.hovering_button = false;
                }

                if temp_button.hovering_button && self.hovering_all_buttons && game.mouse_button == MouseButton::Left {
                    temp_button.clicked = true;
                    self.current_button_clicked_seed = button_index;
                }
                if game.seed_outline_visible && temp_button.clicked {
                    game.current_seed = button_index;
                    Self::draw_rect_outline(game, temp_button.rect);
                    /*                     println!("CURRENT CROP: {}", game.current_crop); */
                }
            }
        }
        Ok(())
    }
    pub fn render_build_buttons (&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, game: &mut game_manager::GameManager) -> Result<(), String> {
        if game.build_mode {
            for (_, temp_button) in self.button_vec.iter_mut().enumerate() {
                if Rect::contains_point(&temp_button.rect, game.mouse_point) {
                    self.hovering_all_buttons = true;
                    break
                }
                else {
                    self.hovering_all_buttons = false;
                }
            }

            for (button_index, mut temp_button) in self.button_vec.iter_mut().enumerate() {
                temp_button.rect = Rect::new(
                    player.rect.x() + constants::TILE_SIZE as i32 * button_index as i32 - constants::SCREEN_WIDTH / 2 + constants::TILE_SIZE as i32,
                    constants::TILE_SIZE as i32 + player.rect.y() - constants::SCREEN_HEIGHT / 2 + constants::TILE_SIZE as i32,
                    constants::TILE_SIZE,
                    constants::TILE_SIZE,
                );
                let texture = tex_man.load(&temp_button.texture_path)?;
                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    temp_button.rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;

                if Rect::contains_point(&temp_button.rect, game.mouse_point) {
                    temp_button.hovering_button = true;
                }
                else {
                    temp_button.hovering_button = false;
                }

                if temp_button.hovering_button && self.hovering_all_buttons && game.mouse_button == MouseButton::Left {
                    temp_button.clicked = true;
                    self.current_button_clicked_build = button_index;
                }
                if game.build_outline_visible && temp_button.clicked {
                    game.current_build = button_index;
                    Self::draw_rect_outline(game, temp_button.rect);
                }
            }
        }
        Ok(())
    }

}
