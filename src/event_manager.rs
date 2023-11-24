use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use crate::game_manager;
use crate::button_manager;
use crate::gui_manager;
use crate::player_manager;
use crate::tower_manager;

pub struct EventManager {
    event_pump: EventPump,
    pub timer_subsystem: sdl2::TimerSubsystem,
    pub current_performance_counter: u64,
    pub last_performance_counter: u64,
    pub performance_frequency: u64,
    pub delta_time: f64,
    pub screen_size: (i32, i32),
    pub mouse_point: sdl2::rect::Point,
    pub level_updated: bool,
    pub menu_settings: bool,
    pub menu_quit: bool,
    pub game_quit: bool,
    pub game_paused: bool,
    pub game_saving: bool,
    pub game_loading: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,

}

impl EventManager {
    pub fn new(sdl_context: &sdl2::Sdl, game: &mut game_manager::GameManager) -> EventManager {
        let event = EventManager {  
            event_pump: sdl_context.event_pump().unwrap(),
            timer_subsystem: sdl_context.timer().unwrap(),
            current_performance_counter: 0,
            last_performance_counter: 0,
            performance_frequency: 0,
            delta_time: 0.0,
            screen_size: (
                game.canvas.window().display_mode().unwrap().w,
                game.canvas.window().display_mode().unwrap().h
            ),
            mouse_point: sdl2::rect::Point::new(0, 0),
            level_updated: false,
            menu_settings: false,
            menu_quit: false,
            game_quit: false,
            game_paused: true,
            game_saving: false,
            game_loading: false,
            up: false,
            down: false,
            left: false,
            right: false,
        };
        event
    }
    pub fn quit_game (
        &mut self, 
        game: &mut game_manager::GameManager, 
    ) {
        self.game_quit = true;
        //ADD AUTOSAVE
    }
    pub fn do_event(
        &mut self, 
        game: &mut game_manager::GameManager, 
        towers: &mut tower_manager::TowerManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
        gui_manager: &mut gui_manager::GUIManager,
    ) {

        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    self.game_quit = true;
                    break
                }
                sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.do_key_down(game, towers, seed_buttons, build_buttons, gui_manager, keycode);
                    break
                }, 
                sdl2::event::Event::KeyUp {keycode: Some(keycode), .. } => {
                    self.do_key_up(keycode);
                    break
                },
                sdl2::event::Event::MouseMotion { x, y, .. } => {
                    game.mouse_point.x = x;
                    game.mouse_point.y = y;
                    self.mouse_point.x = x;
                    self.mouse_point.y = y;
                },
                sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                    game.mouse_button = mouse_btn;
                },
                sdl2::event::Event::MouseButtonUp { .. } => {
                    game.mouse_button = MouseButton::Unknown;
                    game.placed = false;
                },
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(width, height),
                    ..
                } => {
                    self.screen_size = (
                        game.canvas.window().display_mode().unwrap().w,
                        game.canvas.window().display_mode().unwrap().h
                    );
                }
                _ => {}
            }
        }
    }

    fn do_key_down(&mut self, 
        game: &mut game_manager::GameManager, 
        towers: &mut tower_manager::TowerManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
        gui_manager: &mut gui_manager::GUIManager,
        keycode: sdl2::keyboard::Keycode,
    ) {
        match keycode {
            sdl2::keyboard::Keycode::L => {
                self.menu_quit = !self.menu_quit;
                if !self.menu_quit {
                    self.game_paused = true;
                }
                else {
                    self.game_paused = false;
                }
                println!("Paused: {}", self.game_paused);
            },
            sdl2::keyboard::Keycode::P => self.game_paused = !self.game_paused,
            sdl2::keyboard::Keycode::O => {
                if self.game_paused && !self.game_saving && !self.game_loading {
                    self.game_saving = true;
                }
            }
            sdl2::keyboard::Keycode::I => {
                if self.game_paused && !self.game_saving && !self.game_loading {
                    self.game_loading = true;
                }
            },
            sdl2::keyboard::Keycode::Escape => { 
                self.menu_quit = !self.menu_quit;
                if !self.menu_quit {
                    self.game_paused = true;
                }
                else {
                    self.game_paused = false;
                }
                println!("Paused: {}", self.game_paused);
            },
            sdl2::keyboard::Keycode::W => self.up = true,
            sdl2::keyboard::Keycode::S => self.down = true,
            sdl2::keyboard::Keycode::A => self.left = true,
            sdl2::keyboard::Keycode::D => self.right = true,
            sdl2::keyboard::Keycode::T => {
                if game.build_mode {
                    game.build_mode = false;
                }
                else {
                    game.build_mode = true;
                    game.seed_mode = false;
                    game.upgrade_mode = false;
                    return
                }
            },
            sdl2::keyboard::Keycode::Y => {
                if game.seed_mode {
                    game.seed_mode = false;
                }
                else {
                    game.seed_mode = true;
                    game.build_mode = false;
                    game.upgrade_mode = false;
                    return
                }
            },
            sdl2::keyboard::Keycode::M => {
                if sdl2::mixer::Music::get_volume() != 0 {
                    sdl2::mixer::Music::set_volume(0);
                }
                else {
                    sdl2::mixer::Music::set_volume(50);
                }
            },
            sdl2::keyboard::Keycode::Num1 => {
                if game.seed_mode {
                    game.current_seed = 0;
                    if !seed_buttons.button_vec[0].outline_visible {
                        seed_buttons.button_vec[0].outline_visible = true;
                    }
                    else {
                        seed_buttons.button_vec[0].outline_visible = false;
                    }
                    seed_buttons.update_buttons(0, game);
                }
                if game.build_mode {
                    game.current_build = 0;
                    if !build_buttons.button_vec[0].outline_visible {
                        build_buttons.button_vec[0].outline_visible = true;
                    }
                    else {
                        build_buttons.button_vec[0].outline_visible = false;
                    }
                    build_buttons.update_buttons(0, game);
                }
            },
            sdl2::keyboard::Keycode::Num2 => {
                if game.seed_mode {
                    game.current_seed = 1;
                    if !seed_buttons.button_vec[1].outline_visible {
                        seed_buttons.button_vec[1].outline_visible = true;
                    }
                    else {
                        seed_buttons.button_vec[1].outline_visible = false;

                    }
                    seed_buttons.update_buttons(1, game);
                }
                if game.build_mode {
                    game.current_build = 1;
                    if !build_buttons.button_vec[1].outline_visible {
                        build_buttons.button_vec[1].outline_visible = true;
                    }
                    else {
                        build_buttons.button_vec[1].outline_visible = false;
                    }
                    build_buttons.update_buttons(1, game);
                }
            },
            sdl2::keyboard::Keycode::Num3 => {
                if game.seed_mode {
                    game.current_seed = 2;
                    if !seed_buttons.button_vec[2].outline_visible {
                        seed_buttons.button_vec[2].outline_visible = true;
                    }
                    else {
                        seed_buttons.button_vec[2].outline_visible = false;

                    }
                    seed_buttons.update_buttons(2, game);
                }
                if game.build_mode {
                    game.current_build = 2;
                    if !build_buttons.button_vec[2].outline_visible {
                        build_buttons.button_vec[2].outline_visible = true;
                    }
                    else {
                        build_buttons.button_vec[2].outline_visible = false;
                    }
                    build_buttons.update_buttons(2, game);
                }
            },
            sdl2::keyboard::Keycode::Num4 => {
                if game.seed_mode {
                    game.current_seed = 3;
                    if !seed_buttons.button_vec[3].outline_visible {
                        seed_buttons.button_vec[3].outline_visible = true;
                    }
                    else {
                        seed_buttons.button_vec[3].outline_visible = false;

                    }
                    seed_buttons.update_buttons(3, game);
                }
                if game.build_mode {
                    game.current_build = 3;
                    if !build_buttons.button_vec[3].outline_visible {
                        build_buttons.button_vec[3].outline_visible = true;
                    }
                    else {
                        build_buttons.button_vec[3].outline_visible = false;
                    }
                    build_buttons.update_buttons(3, game);
                }
            },
            sdl2::keyboard::Keycode::Num5 => {
                if game.seed_mode {
                    game.current_seed = 4;
                    if !seed_buttons.button_vec[4].outline_visible {
                        seed_buttons.button_vec[4].outline_visible = true;
                    }
                    else {
                        seed_buttons.button_vec[4].outline_visible = false;

                    }
                    seed_buttons.update_buttons(4, game);
                }
                if game.build_mode {
                    game.current_build = 4;
                    if !build_buttons.button_vec[4].outline_visible {
                        build_buttons.button_vec[4].outline_visible = true;
                    }
                    else {
                        build_buttons.button_vec[4].outline_visible = false;
                    }
                    build_buttons.update_buttons(4, game);
                }
            },
            _ => gui_manager.create_message("invalid input".to_string(), 128),
        }
    }

    fn do_key_up(&mut self, 
        keycode: sdl2::keyboard::Keycode
    ) {
        match keycode {
            sdl2::keyboard::Keycode::W => self.up = false,
            sdl2::keyboard::Keycode::S => self.down = false,
            sdl2::keyboard::Keycode::A => self.left = false,
            sdl2::keyboard::Keycode::D => self.right = false,
            _ => {}
        }
    }
}
