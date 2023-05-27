use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use crate::level_manager;
use crate::texture_manager;
use crate::game_manager;
use crate::level_manager::LevelManager;

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;

const IMAGE_WIDTH:u32 = 32;
const IMAGE_HEIGHT:u32 = 32;
const IMAGE_SCALING:u32 = 1;

const OUTPUT_WIDTH: u32 = IMAGE_WIDTH * IMAGE_SCALING;
const OUTPUT_HEIGHT: u32 = IMAGE_HEIGHT * IMAGE_SCALING;

const PLAYER_SPEED: i32 = 32;
const PLAYER_VELOCITY: i32 = 10;
const PLAYER_MAX_VELOCITY: i32 = 40;

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
    pub x_vel: i32,
    pub y_vel: i32,
    pub texture_path: String,
    pub rect: sdl2::rect::Rect,
    pub direction: Direction,
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
            x_vel: 0,
            y_vel: 0,
            texture_path: "".to_string(),
            rect: Rect::new(0, 0, 0, 0),
            direction: Direction::None,
        };
        player
    }

    pub fn update_player(&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, level: &mut level_manager::LevelManager) {
        let original_x: i32 = self.x;
        let original_y: i32 = self.y;
        let mut new_x: i32 = self.x;
        let mut new_y: i32 = self.y; 

        /*   println!("COLLIDING BEFORE: {}, oX: {}, oY: {}, nX: {}, nY: {}", self.colliding, original_x, original_y, new_x, new_y); */
        if !self.colliding {
            if game.up {
                new_y -= PLAYER_SPEED;
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
                new_y += PLAYER_SPEED;
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
                new_x -= PLAYER_SPEED;
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
                new_x += PLAYER_SPEED;
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

        /*  println!("COLLIDING AFTER: {}, oX: {}, oY: {}, nX: {}, nY: {}, sX: {}, sY: {}", self.colliding, original_x, original_y, new_x, new_y, self.x, self.y); */

        if self.colliding {
            match self.direction {
                Direction::Up => {
                    self.y += original_y - new_y + PLAYER_SPEED;
                    self.colliding = false;
                    println!("Up");
                }
                Direction::Down => {
                    self.y += original_y - new_y - PLAYER_SPEED;
                    self.colliding = false;
                    println!("Down");
                }
                Direction::Left => {
                    self.x += original_x - new_x + PLAYER_SPEED;
                    self.colliding = false;
                    println!("Left");
                }
                Direction::Right => {
                    self.x += original_x - new_x - PLAYER_SPEED;
                    self.colliding = false;
                    println!("Right");
                }
                Direction::UpLeft => {
                    self.y += original_y - new_y + PLAYER_SPEED;
                    self.x += original_y - new_y + PLAYER_SPEED;
                    self.colliding = false;
                    println!("UpLeft");
                }
                Direction::UpRight => {
                    self.y += original_y - new_y + PLAYER_SPEED;
                    self.x += original_y - new_y - PLAYER_SPEED;
                    self.colliding = false;
                    println!("UpRight");
                }
                Direction::DownLeft => {
                    self.y += original_y - new_y - PLAYER_SPEED;
                    self.x += original_x - new_x + PLAYER_SPEED;
                    self.colliding = false;
                    println!("DownLeft");
                }
                Direction::DownRight => {
                    self.y += original_y - new_y - PLAYER_SPEED;
                    self.x += original_x - new_x - PLAYER_SPEED;
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
        // let screen_offset_x = SCREEN_WIDTH / 2;
        // let screen_offset_y = SCREEN_HEIGHT / 2;
        // let snapped_x = (self.x - game.cam_x + screen_offset_x) / 32 * 32;
        // let snapped_y = (self.y - game.cam_y + screen_offset_y) / 32 * 32;
        let snapped_x = ((SCREEN_WIDTH / 2) - (self.x - game.cam_x)) / 32 * 32;
        let snapped_y = ((SCREEN_HEIGHT / 2) - (self.y - game.cam_y)) / 32 * 32;

        self.rect = Rect::new(snapped_x, snapped_y, OUTPUT_WIDTH, OUTPUT_HEIGHT);   

        match self.direction {
            Direction::Up => {
                self.texture_path = "assets/player0-back.png".to_string();               
            }
            Direction::Down => {
                self.texture_path = "assets/player0-front.png".to_string();           
            }
            Direction::Left => {
                self.texture_path = "assets/player0-left.png".to_string();
            }
            Direction::Right => {
                self.texture_path = "assets/player0-right.png".to_string();
            }
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
