use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::video::WindowContext;

use crate::level_manager;
use crate::texture_manager;
use crate::game_manager;
use crate::level_manager::LevelManager;

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;

// const IMAGE_WIDTH:u32 = 38;
// const IMAGE_HEIGHT:u32 = 48;
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
}


pub struct PlayerManager {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub colliding: bool,
    pub colliding_up: bool,
    pub colliding_down: bool,
    pub colliding_left: bool,
    pub colliding_right: bool,
    pub x: i32,
    pub y: i32,
    pub x_vel: i32,
    pub y_vel: i32,
    pub texture_path: String,
    pub rect: sdl2::rect::Rect,
}

impl PlayerManager {
    pub fn new() -> PlayerManager {

        let player = PlayerManager {
            up: false,
            down: false,
            left: false,
            right: false,
            colliding: false,
            colliding_up: false,
            colliding_down: false,
            colliding_left: false,
            colliding_right: false,
            x: 0,
            y: 0,
            x_vel: 0,
            y_vel: 0,
            texture_path: "".to_string(),
            rect: Rect::new(0, 0, 0, 0),
        };
        player
    }

    pub fn update_player(&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, level: &mut level_manager::LevelManager) {
        let mut new_x: i32 = self.x;
        let mut new_y: i32 = self.y;
        let mut direction: Option<Direction> = None;

        if game.up && !self.colliding_up {
            self.colliding_down = false;
            self.colliding_left = false;
            self.colliding_right = false;
            direction = Some(Direction::Up);
            new_y -= PLAYER_SPEED;
        } 
        else if game.down && !self.colliding_down {
            self.colliding_up = false;
            self.colliding_left = false;
            self.colliding_right = false;
            direction = Some(Direction::Down);
            new_y += PLAYER_SPEED;
        }
        else if game.left && !self.colliding_left {
            self.colliding_up = false;
            self.colliding_down = false;
            self.colliding_right = false;
            direction = Some(Direction::Left);
            new_x -= PLAYER_SPEED;
        }
        else if game.right && !self.colliding_right {
            self.colliding_up = false;
            self.colliding_down = false;
            self.colliding_left = false;
            direction = Some(Direction::Right);
            new_x += PLAYER_SPEED;
        }

        let collision = LevelManager::check_all_collisions(level, self);

        if collision {
            match direction {
                Some(Direction::Up) => {
                    if LevelManager::check_all_collisions(level, self) {
                        println!("COLLIDING UP");
                        self.colliding_up = true;
                        self.y += PLAYER_SPEED;
                        return
                    }
                }
                Some(Direction::Down) => {
                    if LevelManager::check_all_collisions(level, self) {
                        println!("COLLIDING DOWN");
                        self.colliding_down = true;
                        self.y -= PLAYER_SPEED;
                        return
                    }
                }
                Some(Direction::Left) => {
                    if LevelManager::check_all_collisions(level, self) {
                        println!("COLLIDING LEFT");
                        self.colliding_left = true;
                        self.x += PLAYER_SPEED;
                        return
                    }
                }
                Some(Direction::Right) => {
                    if LevelManager::check_all_collisions(level, self) {
                        println!("COLLIDING RIGHT");
                        self.colliding_right = true;
                        self.x -= PLAYER_SPEED;
                        return
                    }
                }
                _ => {}
            }
        }

        self.x = new_x;
        self.y = new_y;
    }
    pub fn render_player(&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<WindowContext>) -> Result<(), String> {
        // let screen_offset_x = SCREEN_WIDTH / 2;
        // let screen_offset_y = SCREEN_HEIGHT / 2;
        // let snapped_x = (self.x - game.cam_x + screen_offset_x) / 32 * 32;
        // let snapped_y = (self.y - game.cam_y + screen_offset_y) / 32 * 32;
        let snapped_x = ((SCREEN_WIDTH / 2) - (self.x - game.cam_x)) / 32 * 32;
        let snapped_y = ((SCREEN_HEIGHT / 2) - (self.y - game.cam_y)) / 32 * 32;

        self.rect = Rect::new(snapped_x, snapped_y, OUTPUT_WIDTH, OUTPUT_HEIGHT);        
        self.texture_path = "assets/player1.png".to_string();
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
