use crate::{player_manager, constants, texture_manager, game_manager, event_manager};

pub enum ButtonType {
    Seed,
    Build,
}

pub struct Button {
    rect: sdl2::rect::Rect,
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

    fn create_seed_buttons(&mut self, player: &player_manager::PlayerManager) {
        for button_index in 0..self.button_amount {
            let temp_button = self::Button {
                rect: sdl2::rect::Rect::new(player.x, player.y, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: match button_index {
                    constants::CURRENT_SEED_SHOVEL => constants::TEXTURE_BUTTON_SHOVEL.to_string(),
                    constants::CURRENT_SEED_HO => constants::TEXTURE_BUTTON_HO.to_string(),
                    constants::CURRENT_SEED_CARROT => constants::TEXTURE_BUTTON_CARROT.to_string(),
                    constants::CURRENT_SEED_TOMATO => constants::TEXTURE_BUTTON_TOMATO.to_string(),
                    _ => constants::TEXTURE_DEFAULT.to_string(),
                },
                clicked: false,
                hovering_button: false,
                outline_visible: false,
            };
            self.button_vec.push(temp_button);
        }

    }
    fn create_build_buttons(&mut self, player: &player_manager::PlayerManager) {
        for button_index in 0..self.button_amount {
            let temp_button = self::Button {
                rect: sdl2::rect::Rect::new(player.x, player.y, constants::TILE_SIZE, constants::TILE_SIZE),
                texture_path: match button_index {
                    constants::CURRENT_BUILD_ARCHER_TOWER => constants::TEXTURE_BUTTON_ARCHER.to_string(),
                    constants::CURRENT_BUILD_FIREBALL_TOWER => constants::TEXTURE_PROJECTILE_FIREBALL.to_string(),
                    constants::CURRENT_BUILD_GOBLIN => constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string(),
                    constants::CURRENT_BUILD_WALL => constants::TEXTURE_TILE_WALL.to_string(),
                    constants::CURRENT_BUILD_BASE => constants::TEXTURE_BUILDING_HOUSE.to_string(),
                    _ => constants::TEXTURE_DEFAULT.to_string(),
                },
                clicked: false,
                hovering_button: false,
                outline_visible: false,
            };
            self.button_vec.push(temp_button);
        }
    }

    fn draw_rect_outline(game: &mut game_manager::GameManager, rect: sdl2::rect::Rect) {
        game.canvas.set_draw_color(constants::COLOR_OUTLINE);
        game.canvas.draw_line(rect.top_left(), rect.top_right()).unwrap();
        game.canvas.draw_line(rect.bottom_left(), rect.bottom_right()).unwrap();
        game.canvas.draw_line(rect.top_left(), rect.bottom_left()).unwrap();
        game.canvas.draw_line(rect.top_right(), rect.bottom_right()).unwrap();
    }

    pub fn update_buttons (&mut self, button_index: usize, game: &mut game_manager::GameManager) {
        let button = &mut self.button_vec[button_index];
        if sdl2::rect::Rect::contains_point(&button.rect, game.mouse_point) {
            button.hovering_button = true;
        }
        else {
            button.hovering_button = false;
        }
        if button.hovering_button && game.mouse_button == sdl2::mouse::MouseButton::Left {
            button.clicked = true;
        }
        else {
            button.clicked = false;
        }
        if button.clicked {
            if game.build_mode {
                game.current_build = button_index;
            }
            else if game.seed_mode {
                game.current_seed = button_index;
            }
            button.outline_visible = true;
        }
        if button.outline_visible {
            Self::draw_rect_outline(game, button.rect);
            for other_button_index in 0..self.button_vec.len() {
                let other_button = &mut self.button_vec[other_button_index];
                if other_button_index != button_index {
                    if other_button.clicked {
                        other_button.clicked = false;
                    }
                    if other_button.outline_visible {
                        other_button.outline_visible = false;
                    }
                }
                if other_button.hovering_button {
                    game.hovering_button = true;
                }
            }
        }
        if !self.button_vec.iter().any(|button| button.hovering_button) {
            game.hovering_button = false;
        }
    }

    pub fn render_build_buttons (&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, events: &mut event_manager::EventManager, game: &mut game_manager::GameManager) -> Result<(), String> {
        if game.build_mode {
            for button_index in 0..self.button_vec.len() {
                self.button_vec[button_index].rect.set_x(player.rect.x() + constants::TILE_SIZE as i32 * button_index as i32 - events.screen_size.0 / 2 + constants::TILE_SIZE as i32);
                self.button_vec[button_index].rect.set_y(constants::TILE_SIZE as i32 + player.rect.y() - events.screen_size.1 / 2 + constants::TILE_SIZE as i32);

                let texture = tex_man.load(&self.button_vec[button_index].texture_path)?;
                game.canvas.copy_ex(
                    &texture,
                    None,
                    self.button_vec[button_index].rect,
                    0.0,
                    None,
                    false,
                    false,
                )?;
                self.update_buttons(button_index, game);
            }
        }
        Ok(())
    }

    pub fn render_seed_buttons (&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, events: &mut event_manager::EventManager, game: &mut game_manager::GameManager) -> Result<(), String> {
        if game.seed_mode {
            for button_index in 0..self.button_vec.len() {
                self.button_vec[button_index].rect.set_x(player.rect.x() + constants::TILE_SIZE as i32 * button_index as i32 - events.screen_size.0 as i32 / 2 + constants::TILE_SIZE as i32);
                self.button_vec[button_index].rect.set_y(constants::TILE_SIZE as i32 + player.rect.y() - events.screen_size.1 / 2 + constants::TILE_SIZE as i32);

                let texture = tex_man.load(&self.button_vec[button_index].texture_path)?;
                game.canvas.copy_ex(
                    &texture,
                    None,
                    self.button_vec[button_index].rect,
                    0.0,
                    None,
                    false,
                    false,
                )?;
                self.update_buttons(button_index, game);
            }
        }
        Ok(())
    }
}
