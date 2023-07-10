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
    pub mouse_point: sdl2::rect::Point,
    pub menu_quit: bool,
    pub game_quit: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,

}

impl EventManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> EventManager {
        let event = EventManager {  
            event_pump: sdl_context.event_pump().unwrap(),
            timer_subsystem: sdl_context.timer().unwrap(),
            current_performance_counter: 0,
            last_performance_counter: 0,
            performance_frequency: 0,
            delta_time: 0.0,
            mouse_point: sdl2::rect::Point::new(0, 0),
            menu_quit: false,
            game_quit: false,
            up: false,
            down: false,
            left: false,
            right: false,
        };
        event
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
                Event::Quit {..} => {
                    self.game_quit = true;
                    break
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.do_key_down(game, towers, seed_buttons, build_buttons, gui_manager, keycode);
                    break
                }, 
                Event::KeyUp {keycode: Some(keycode), .. } => {
                    self.do_key_up(game, keycode);
                    break
                },
                Event::MouseMotion { x, y, .. } => {
                    game.mouse_point.x = x;
                    game.mouse_point.y = y;
                    self.mouse_point.x = x;
                    self.mouse_point.y = y;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    game.mouse_button = mouse_btn;
                }
                Event::MouseButtonUp { .. } => {
                    game.mouse_button = MouseButton::Unknown;
                    game.placed = false;
                },
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
                    game.paused_state = true;
                }
                else {
                    game.paused_state = false;
                }
            }
            sdl2::keyboard::Keycode::P => game.paused_state = !game.paused_state,
            sdl2::keyboard::Keycode::O => {
                if game.paused_state {
                    game.saving = true;
                }
            }
            sdl2::keyboard::Keycode::I => {
                if game.paused_state && !game.saving {
                    game.loading = true;
                }
            }
            sdl2::keyboard::Keycode::Escape => self.game_quit = true,
            /*             sdl2::keyboard::Keycode::Q => game.quit = true, */
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
                    return
                }
            }
            sdl2::keyboard::Keycode::Y => {
                if game.seed_mode {
                    game.seed_mode = false;
                }
                else {
                    game.seed_mode = true;
                    game.build_mode = false;
                    return
                }
            }
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
        game: &mut game_manager::GameManager, 
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
