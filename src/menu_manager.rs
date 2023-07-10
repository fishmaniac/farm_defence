use crate::game_manager;
use crate::constants;

pub struct MenuButton <'a>{
    rect: sdl2::rect::Rect,
    button_text: String,
    pub clicked: bool,
    pub hovering_button: bool,
    pub outline_visible: bool,
    pub texture_surface: sdl2::surface::Surface<'a>,
}

pub struct MenuManager <'a> {
    pub quit: bool,
    pub button_vec: Vec<MenuButton<'a>>,
    pub button_amount: usize,
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub font: &'a sdl2::ttf::Font<'a, 'a>,
}

impl<'a> MenuManager<'a> {
    pub fn new (game: &mut game_manager::GameManager, font: &'a sdl2::ttf::Font<'a, 'a>) -> MenuManager<'a> {
        let menu = MenuManager {
            quit: false,
            button_vec: Vec::new(),
            button_amount: 3,
            texture_creator: game.canvas.texture_creator(),
            font,
        };
        menu
    }
    pub fn create_menu (&mut self) -> Result<(), String> {
        let texture_surface = self.font.render(&"test2".to_string())
            .blended(constants::COLOR_WHITE)
            .map_err(|e| e.to_string())?;
        let button = self::MenuButton {
            rect: sdl2::rect::Rect::new(0, 0, constants::TILE_SIZE, constants::TILE_SIZE),
            button_text: "test".to_string(),
            clicked: false,
            hovering_button: false,
            outline_visible: false,
            texture_surface,
        };
        self.button_vec.push(button);
        Ok(())
    }

    pub fn render_menu (&mut self, game: &mut game_manager::GameManager) -> Result<(), String> {
        for menu_button_index in 0..self.button_vec.len() {
            let menu_button = &mut self.button_vec[menu_button_index];
            menu_button.rect.set_x(game.screen_size.0 / 2);
            menu_button.rect.set_y(2 * constants::TILE_SIZE as i32 + (constants::TILE_SIZE as i32 * menu_button_index as i32));

            let texture = self.texture_creator.create_texture_from_surface(&menu_button.texture_surface).unwrap();
            let dest = sdl2::rect::Rect::new(game.screen_size.0 / 2 - menu_button.texture_surface.width() as i32 / 2, 2 * constants::TILE_SIZE as i32 + (constants::TILE_SIZE as i32 * menu_button_index as i32), menu_button.texture_surface.width(), menu_button.texture_surface.height());   
            game.canvas.copy(&texture, None, Some(dest)).unwrap(); 
        }
        Ok(())
    }
}
