use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use crate::constants;
use crate::texture_manager;
use crate::game_manager;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    None,
}

pub struct PlayerManager {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub colliding: bool,
    pub x: i32,
    pub y: i32,
    pub texture_path: String,
    pub rect: sdl2::rect::Rect,
    pub direction: Direction,
    pub menu_selection: u8,
}

impl PlayerManager {
    pub fn new() -> PlayerManager {
        let player = PlayerManager {
            up: false,
            down: false,
            left: false,
            right: false,
            colliding: false,
            x: 0,
            y: 0,
            texture_path: "".to_string(),
            rect: Rect::new(0, 0, 0, 0),
            direction: Direction::Up,
            menu_selection: 0,
        };
        player
    }

    pub fn update_player(&mut self, game: &mut game_manager::GameManager) {
        let original_x: i32 = self.x;
        let original_y: i32 = self.y;
        let mut new_x: i32 = self.x;
        let mut new_y: i32 = self.y; 

        if !self.colliding {
            if game.up {
                new_y -= constants::PLAYER_SPEED;
                if game.left && !game.right {
                    self.direction = Direction::UpLeft;
                }
                else if game.right && !game.left {
                    self.direction = Direction::UpRight;
                }
                else {
                    self.direction = Direction::Up;
                }
            } 
            if game.down {
                new_y += constants::PLAYER_SPEED;
                if game.left && !game.right {
                    self.direction = Direction::DownLeft;
                }
                else if game.right && !game.left {
                    self.direction = Direction::DownRight;
                }
                else {
                    self.direction = Direction::Down;
                }
            }
            if game.left {
                new_x -= constants::PLAYER_SPEED;
                if game.up && !game.down {
                    self.direction = Direction::UpLeft;
                }
                else if game.down && !game.up {
                    self.direction = Direction::DownLeft;
                }
                else {
                    self.direction = Direction::Left;
                }
            }
            if game.right {
                new_x += constants::PLAYER_SPEED;
                if game.up && !game.down {
                    self.direction = Direction::UpRight;
                }
                else if game.down && !game.up {
                    self.direction = Direction::DownRight;
                }
                else {
                    self.direction = Direction::Right;
                }
            }
        }

        if self.colliding {
            match self.direction {
                Direction::Up => {
                    self.y += original_y - new_y + constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("Up");
                }
                Direction::Down => {
                    self.y += original_y - new_y - constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("Down");
                }
                Direction::Left => {
                    self.x += original_x - new_x + constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("Left");
                }
                Direction::Right => {
                    self.x += original_x - new_x - constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("Right");
                }
                Direction::UpLeft => {
                    self.y += original_y - new_y + constants::PLAYER_SPEED;
                    self.x += original_y - new_y + constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("UpLeft");
                }
                Direction::UpRight => {
                    self.y += original_y - new_y + constants::PLAYER_SPEED;
                    self.x += original_y - new_y - constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("UpRight");
                }
                Direction::DownLeft => {
                    self.y += original_y - new_y - constants::PLAYER_SPEED;
                    self.x += original_x - new_x + constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("DownLeft");
                }
                Direction::DownRight => {
                    self.y += original_y - new_y - constants::PLAYER_SPEED;
                    self.x += original_x - new_x - constants::PLAYER_SPEED;
                    self.colliding = false;
                    println!("DownRight");
                }
                _=> {
                    println!("ERROR IN DIRECTION");
                }
            }
        }
        else {
            self.x = new_x;
            self.y = new_y;
        }
    }
    pub fn render_player(&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<WindowContext>) -> Result<(), String> {
        let snapped_x = ((constants::SCREEN_WIDTH / 2) - (self.x - game.cam_x)) / 32 * 32;
        let snapped_y = ((constants::SCREEN_HEIGHT / 2) - (self.y - game.cam_y)) / 32 * 32;

        self.rect = Rect::new(snapped_x, snapped_y, constants::OUTPUT_WIDTH, constants::OUTPUT_HEIGHT);   

        match self.direction {
            Direction::Up => self.texture_path = "assets/player0-back.png".to_string(),
            Direction::Down => self.texture_path = "assets/player0-front.png".to_string(),
            Direction::Left => self.texture_path = "assets/player0-left.png".to_string(),
            Direction::Right => self.texture_path = "assets/player0-right.png".to_string(),
            _ => {
                println!("NO PLAYER TEXTURE");
                self.texture_path = "assets/player0-front.png".to_string();
            }
        }

        let texture = tex_man.load(&self.texture_path)?;

        game.canvas.copy_ex(
            &texture, // Texture object
            None,      // source rect
            self.rect,     // destination rect
            0.0,      // angle (degrees)
            None,   // center
            false,    // flip horizontal
            false     // flip vertical
        )?;

        Ok(())
    }

}
