use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use crate::game_manager::GameManager;
pub struct EventManager {
    event_pump: EventPump,
}

impl EventManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> EventManager {
        let event_pump = sdl_context.event_pump().unwrap(); 
        let event = EventManager {  
            event_pump,
        };
        event
    }

    pub fn do_event(&mut self, game: &mut GameManager) {
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
                Event::MouseMotion { x, y, .. } => {
                    game.mouse_point.x = x;
                    game.mouse_point.y = y;
                    println!("MOUSE MOVED: x={}, y={}", game.mouse_point.x(), game.mouse_point.y());
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    game.mouse_button = mouse_btn;
                    let mouse_btn_str = match game.mouse_button {
                        MouseButton::Unknown => "Unknown",
                        MouseButton::Left => "Left",
                        MouseButton::Middle => "Middle",
                        MouseButton::Right => "Right",
                        MouseButton::X1 => "X1",
                        MouseButton::X2 => "X2",
                    };
                    println!("MOUSE CLICKED: {}", mouse_btn_str);
                }
                Event::MouseButtonUp { .. } => game.mouse_button = MouseButton::Unknown,
                _ => {}
            }
        }
    }

    fn do_key_down(&mut self, game: &mut GameManager, keycode: sdl2::keyboard::Keycode) {
        match keycode {
            sdl2::keyboard::Keycode::Escape => game.quit = true,
            sdl2::keyboard::Keycode::Q => game.quit = true,
            sdl2::keyboard::Keycode::W => game.up = true,
            sdl2::keyboard::Keycode::S => game.down = true,
            sdl2::keyboard::Keycode::A => game.left = true,
            sdl2::keyboard::Keycode::D => game.right = true,
            sdl2::keyboard::Keycode::T => {
                if game.placing {
                    game.placing = false;
                }
                else {
                    game.placing = true;
                    println!("PLACING: {}", game.placing);
                    return
                }
                println!("PLACING: {}", game.placing);
            }
            _ => println!("INVALID INPUT"),
        }
    }

    fn do_key_up(&mut self, game : &mut GameManager, keycode: sdl2::keyboard::Keycode) {
        match keycode {
            sdl2::keyboard::Keycode::W => game.up = false,
            sdl2::keyboard::Keycode::S => game.down = false,
            sdl2::keyboard::Keycode::A => game.left = false,
            sdl2::keyboard::Keycode::D => game.right = false,
            _ => {}
        }
    }
}
