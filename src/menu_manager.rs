use crate::event_manager;
use crate::game_manager;
use crate::constants;
use crate::player_manager;

pub struct MenuButton <'a> {
    texture_surface: sdl2::surface::Surface<'a>,
    rect: sdl2::rect::Rect,
    button_text: String,
    clicked: bool,
    hovering_button: bool,
    outline_visible: bool,
    last_clicked: i32,
}

pub struct MenuManager <'a> {
    pub quit: bool,
    button_vec: Vec<MenuButton<'a>>,
    settings_vec: Vec<MenuButton<'a>>,
    resolution_vec: Vec<(u32, u32)>,
    button_amount: usize,
    current_resolution: usize,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    small_font: &'a sdl2::ttf::Font<'a, 'a>,
    large_font: &'a sdl2::ttf::Font<'a, 'a>,

}

impl<'a> MenuManager<'a> {
    pub fn new (game: &mut game_manager::GameManager, small_font: &'a sdl2::ttf::Font<'a, 'a>, large_font: &'a sdl2::ttf::Font<'a, 'a>) -> MenuManager<'a> {
        let menu = MenuManager {
            quit: false,
            button_vec: Vec::new(),
            settings_vec: Vec::new(),
            resolution_vec: vec![(640, 480), (1280, 720), (1920, 1080), (2560, 1440), (4096, 2304)],
            current_resolution: 0,
            button_amount: 3,
            texture_creator: game.canvas.texture_creator(),
            small_font,
            large_font,
        };
        menu
    }
    pub fn create_menu (&mut self, game: &mut game_manager::GameManager, events: &mut event_manager::EventManager) -> Result<(), String> {
        let texture_surface = self.large_font.render(&"Farm Defense".to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let main_text = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };
        let texture_surface = self.large_font.render(&"Play".to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let play_button = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };
        let texture_surface = self.large_font.render(&"Settings".to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let settings_button = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };

        let texture_surface = self.large_font.render(&"Resolution".to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let resolution_button = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };
        let texture_surface = self.large_font.render(&format!("{} x {}", events.screen_size.0, events.screen_size.1).to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let resolution_string = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };
        let texture_surface = self.large_font.render(&"-".to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let resolution_minus = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };
        let texture_surface = self.large_font.render(&"+".to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let resolution_plus = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };
        let texture_surface = self.large_font.render(&format!("BACK").to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let back_button = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };
        let texture_surface;
        match game.canvas.window().fullscreen_state() {
            sdl2::video::FullscreenType::True => {
                texture_surface = self.large_font.render(&"FULLSCREEN".to_string())
                    .blended(constants::COLOR_WHITE)
                    .map_err(|e| e.to_string())?;
            }
            sdl2::video::FullscreenType::Off => {
                texture_surface = self.large_font.render(&"WINDOWED".to_string())
                    .blended(constants::COLOR_WHITE)
                    .map_err(|e| e.to_string())?;
            }
            sdl2::video::FullscreenType::Desktop => {
                texture_surface = self.large_font.render(&"BORDERLESS".to_string())
                    .blended(constants::COLOR_WHITE)
                    .map_err(|e| e.to_string())?;
            }
        }

        let fullscreen_button = self::MenuButton {
            texture_surface,
            rect: sdl2::rect::Rect::new(0, 0, 0, 0),
            button_text: "".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            last_clicked: 0,
        };


        self.button_vec.push(main_text);
        self.button_vec.push(play_button);
        self.button_vec.push(settings_button);
        self.settings_vec.push(resolution_button);
        self.settings_vec.push(resolution_string);
        self.settings_vec.push(fullscreen_button);
        self.settings_vec.push(back_button);
        self.settings_vec.push(resolution_minus);
        self.settings_vec.push(resolution_plus);

        Ok(())
    }

    pub fn render_menu (&mut self, events: &mut event_manager::EventManager, game: &mut game_manager::GameManager, player: &mut player_manager::PlayerManager) -> Result<(), String> {
        if !events.menu_settings {
            for menu_button_index in 0..self.button_vec.len() {
                self.update_menu_buttons(menu_button_index, game);

                let menu_button = &mut self.button_vec[menu_button_index];

                menu_button.rect.set_x(events.screen_size.0 / 2 - menu_button.texture_surface.width() as i32 / 2);
                menu_button.rect.set_y(menu_button.texture_surface.height() as i32 * menu_button_index as i32);
                menu_button.rect.set_width(menu_button.texture_surface.width());
                menu_button.rect.set_height(menu_button.texture_surface.height());


                let texture_result = self.texture_creator.create_texture_from_surface(&menu_button.texture_surface);

                let texture = match texture_result {
                    Ok(texture) => texture,
                    Err(err) => {
                        eprintln!("Failed to create texture from surface:\t{}", err);
                        continue; 
                    }
                };

                if let Err(err) = game.canvas.copy(&texture, None, menu_button.rect) {
                    eprintln!("Failed to copy texture to canvas:\t{}", err);
                }
                if menu_button.clicked && menu_button.last_clicked > 32 {
                    if menu_button_index == constants::CURRENT_BUTTON_MENU_PLAY {
                        events.menu_quit = true;
                        events.game_paused = false;
                    }
                    else if menu_button_index == constants::CURRENT_BUTTON_MENU_SETTINGS
                    {
                        events.menu_settings = true;
                        menu_button.outline_visible = false;
                    }

                    menu_button.last_clicked = 0;
                }
                menu_button.last_clicked += 1;
            }
        }
        let bottom_of_menu = self.button_vec[self.button_vec.len() - 1].rect.y() + self.button_vec[self.button_vec.len() - 1].rect.height() as i32;
        if events.menu_settings {
            for settings_button_index in 0..self.settings_vec.len() {
                self.update_settings_buttons(settings_button_index, game);

                let resolution_rect_data: (i32, i32, u32, u32) = (self.settings_vec[1].rect.x(), self.settings_vec[1].rect.y(), self.settings_vec[1].rect.width(), self.settings_vec[1].rect.height());
                let settings_button = &mut self.settings_vec[settings_button_index];
                
                let texture_result = self.texture_creator.create_texture_from_surface(&settings_button.texture_surface);

                let texture = match texture_result {
                    Ok(texture) => texture,
                    Err(err) => {
                        eprintln!("Failed to create texture from surface:\t{}", err);
                        continue;
                    }
                };

                match settings_button_index {
                    constants::CURRENT_BUTTON_SETTINGS_RESOLUTION_MINUS => {
                        settings_button.rect.set_x(resolution_rect_data.0 - settings_button.texture_surface.width() as i32 - constants::TILE_SIZE as i32);
                        settings_button.rect.set_y(resolution_rect_data.1);
                        settings_button.rect.set_width(settings_button.texture_surface.width());
                        settings_button.rect.set_height(settings_button.texture_surface.height());
                        player.rect.set_x(events.screen_size.0 / 2);
                        player.rect.set_y(events.screen_size.1 / 2);
                    }
                    constants::CURRENT_BUTTON_SETTINGS_RESOLUTION_PLUS => {
                        settings_button.rect.set_x(resolution_rect_data.0 + resolution_rect_data.2 as i32 + constants::TILE_SIZE as i32);
                        settings_button.rect.set_y(resolution_rect_data.1);
                        settings_button.rect.set_width(settings_button.texture_surface.width());
                        settings_button.rect.set_height(settings_button.texture_surface.height());
                        player.rect.set_x(events.screen_size.0 / 2);
                        player.rect.set_y(events.screen_size.1 / 2);
                    }
                    _ => {
                        settings_button.rect.set_x(events.screen_size.0 / 2 - settings_button.texture_surface.width() as i32 / 2);
                        settings_button.rect.set_y(bottom_of_menu + (settings_button.texture_surface.height() as i32 * settings_button_index as i32));
                        settings_button.rect.set_width(settings_button.texture_surface.width());
                        settings_button.rect.set_height(settings_button.texture_surface.height());
                    }

                }

                if let Err(err) = game.canvas.copy(&texture, None, settings_button.rect) {
                    eprintln!("Failed to copy texture to canvas:\t{}", err);
                }
                if settings_button_index == constants::CURRENT_BUTTON_SETTINGS_RESOLUTION_STRING { 
                    events.screen_size = (game.canvas.window().display_mode().unwrap().w, game.canvas.window().display_mode().unwrap().h);
                    let texture_surface = self.large_font.render(&format!("{} x {}", events.screen_size.0, events.screen_size.1).to_string())
                        .blended(constants::COLOR_WHITE)
                        .map_err(|e| e.to_string())?;
                    settings_button.texture_surface = texture_surface;

                }
                if settings_button.clicked && settings_button.last_clicked > 32 {
                    match settings_button_index {
                        constants::CURRENT_BUTTON_SETTINGS_BACK => {
                            events.menu_settings = false;
                        }
                        constants::CURRENT_BUTTON_SETTINGS_RESOLUTION_STRING => {
                        }
                        constants::CURRENT_BUTTON_SETTINGS_RESOLUTION_MINUS => {
                            //ADD ERROR HANDLING
                            if self.current_resolution > 0 {
                                self.current_resolution -= 1;
                            }
                            game.canvas.window_mut().set_size(self.resolution_vec[self.current_resolution].0, self.resolution_vec[self.current_resolution].1);
                        }
                        constants::CURRENT_BUTTON_SETTINGS_RESOLUTION_PLUS => {
                            //ADD ERROR HANDLING
                            if self.current_resolution < self.resolution_vec.len() - 1 {
                                self.current_resolution += 1;
                            }
                            game.canvas.window_mut().set_size(self.resolution_vec[self.current_resolution].0, self.resolution_vec[self.current_resolution].1);
                        }
                        constants::CURRENT_BUTTON_SETTINGS_FULLSCREEN => {

                            let texture_surface: Option<sdl2::surface::Surface> = match game.canvas.window().fullscreen_state() {
                                sdl2::video::FullscreenType::True => {
                                    if let Err(err) = game.canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::Off) {
                                        eprintln!("Failed to set fullscreen mode: {}", err);
                                        None
                                    } 
                                    else {
                                        Some(self.large_font.render(&"WINDOWED".to_string())
                                            .blended(constants::COLOR_WHITE)
                                            .map_err(|e| e.to_string())?)
                                    }
                                },
                                sdl2::video::FullscreenType::Off => {
                                    if let Err(err) = game.canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::Desktop) {
                                        eprintln!("Failed to set fullscreen mode: {}", err);
                                        None
                                    } 
                                    else {
                                        Some(self.large_font.render(&"BORDERLESS".to_string())
                                            .blended(constants::COLOR_WHITE)
                                            .map_err(|e| e.to_string())?)
                                    }
                                },
                                sdl2::video::FullscreenType::Desktop => {
                                    if let Err(err) = game.canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True) {
                                        eprintln!("Failed to set fullscreen mode: {}", err);
                                        None
                                    } 
                                    else {
                                        Some(self.large_font.render(&"FULLSCREEN".to_string())
                                            .blended(constants::COLOR_WHITE)
                                            .map_err(|e| e.to_string())?)
                                    }
                                },
                            };

                            if let Some(texture_surface) = texture_surface {
                                settings_button.texture_surface = texture_surface;
                            }
                        }
                        _ => {},
                    }



                    settings_button.outline_visible = false;
                    settings_button.last_clicked = 0;
                }
                settings_button.last_clicked += 1;
            }

        }
        Ok(())
    }
    fn draw_rect_outline(game: &mut game_manager::GameManager, rect: sdl2::rect::Rect) {
        game.canvas.set_draw_color(constants::COLOR_OUTLINE);
        game.canvas.draw_line(rect.top_left(), rect.top_right()).unwrap();
        game.canvas.draw_line(rect.bottom_left(), rect.bottom_right()).unwrap();
        game.canvas.draw_line(rect.top_left(), rect.bottom_left()).unwrap();
        game.canvas.draw_line(rect.top_right(), rect.bottom_right()).unwrap();
    }

    pub fn update_menu_buttons (&mut self, menu_button_index: usize, game: &mut game_manager::GameManager) {
        let button = &mut self.button_vec[menu_button_index];
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
                game.current_build = menu_button_index;
            }
            else if game.seed_mode {
                game.current_seed = menu_button_index;
            }
            button.outline_visible = true;
        }
        if button.outline_visible {
            Self::draw_rect_outline(game, button.rect);
            for other_button_index in 0..self.button_vec.len() {
                let other_button = &mut self.button_vec[other_button_index];
                if other_button_index != menu_button_index {
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
    pub fn update_settings_buttons (&mut self, settings_button_index: usize, game: &mut game_manager::GameManager) {
        let button = &mut self.settings_vec[settings_button_index];
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
                game.current_build = settings_button_index;
            }
            else if game.seed_mode {
                game.current_seed = settings_button_index;
            }
            button.outline_visible = true;
        }
        if button.outline_visible {
            Self::draw_rect_outline(game, button.rect);
            for other_button_index in 0..self.settings_vec.len() {
                let other_button = &mut self.settings_vec[other_button_index];
                if other_button_index != settings_button_index {
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
        if !self.settings_vec.iter().any(|button| button.hovering_button) {
            game.hovering_button = false;
        }
    }

}
