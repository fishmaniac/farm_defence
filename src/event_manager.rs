use sdl2::EventPump;
use sdl2::event::Event;

use crate::game_manager::GameManager;

//Option allows to hold either Some(event) or None

pub struct EventManager {
    // up: bool,
    // down: bool,
    // left: bool,
    // right: bool,
    // event: Option<EventType>,
    event_pump: EventPump,
}

impl EventManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> EventManager {
        let event_pump = sdl_context.event_pump().unwrap(); 
        let event = EventManager {  
            // up: false,
            // down: false,
            // left: false,
            // right: false,
            // event: None,
            event_pump,
        };
        event
    }

    pub fn do_keyboard_event(&mut self, game: &mut GameManager) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    game.quit = true;
                    break
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.do_key_down(game, keycode);
                    break
                }, 
                Event::KeyUp {keycode: Some(keycode), .. } => {
                    self.do_key_up(game, keycode);
                    break
                },
                _ => {}
            }
        }
    }

    fn do_key_down(&mut self, game: &mut GameManager, keycode: sdl2::keyboard::Keycode) {
        match keycode {
            sdl2::keyboard::Keycode::Escape => {
                game.quit = true;
            },
            sdl2::keyboard::Keycode::Q => {
                game.quit = true;
            }
            sdl2::keyboard::Keycode::W => {
                game.up = true;
            }
            sdl2::keyboard::Keycode::S => {
                game.down = true;
            }
            sdl2::keyboard::Keycode::A => {
                game.left = true;
            }
            sdl2::keyboard::Keycode::D => {
                game.right = true;
            }
            _ => {}
        }
    }

    fn do_key_up(&mut self, game : &mut GameManager, keycode: sdl2::keyboard::Keycode) {
        match keycode {
            sdl2::keyboard::Keycode::W => {
                game.up = false;
            }
            sdl2::keyboard::Keycode::S => {
                game.down = false;
            }
            sdl2::keyboard::Keycode::A => {
                game.left = false;
            }
            sdl2::keyboard::Keycode::D => {
                game.right = false;
            }
            _ => {}
        }
    }
}
