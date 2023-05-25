use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::video::WindowContext;

use crate::texture_manager;
use crate::game_manager;

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;

const IMAGE_WIDTH:u32 = 38;
const IMAGE_HEIGHT:u32 = 48;
const IMAGE_SCALING:u32 = 4;

const OUTPUT_WIDTH: u32 = IMAGE_WIDTH * IMAGE_SCALING;
const OUTPUT_HEIGHT: u32 = IMAGE_HEIGHT * IMAGE_SCALING;

const PLAYER_VELOCITY: i32 = 10;
const PLAYER_MAX_VELOCITY: i32 = 40;


pub struct PlayerManager {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub x: i32,
    pub y: i32,
    pub x_vel: i32,
    pub y_vel: i32,
    pub texture_path: String,
}

impl PlayerManager {
    pub fn new() -> PlayerManager {

        let player = PlayerManager {
            up: false,
            down: false,
            left: false,
            right: false,
            x: 0,
            y: 0,
            x_vel: 0,
            y_vel: 0,
            texture_path: "".to_string(),
        };
        player
    }

    pub fn update_player(&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<WindowContext>) {
        if game.up {
            self.y_vel -= PLAYER_VELOCITY;
        }
        if game.down {
            self.y_vel += PLAYER_VELOCITY;
        }
        if game.left {
            self.x_vel -= PLAYER_VELOCITY;
        }
        if game.right {
            self.x_vel += PLAYER_VELOCITY;
        }

        self.x_vel = self.x_vel.max(-PLAYER_MAX_VELOCITY).min(PLAYER_MAX_VELOCITY);
        self.y_vel = self.y_vel.max(-PLAYER_MAX_VELOCITY).min(PLAYER_MAX_VELOCITY);

        if self.x_vel != 0 {
            self.x += self.x_vel;
            self.x_vel -= self.x_vel.signum() * 5; // Reduce x velocity by 5 units
        }

        if self.y_vel != 0 {
            self.y += self.y_vel;
            self.y_vel -= self.y_vel.signum() * 5; // Reduce y velocity by 5 units
        }
        
        self.render_player(game, tex_man).unwrap();
    }

    fn render_player(&mut self, game: &mut game_manager::GameManager, tex_man: &mut texture_manager::TextureManager<WindowContext>) -> Result<(), String> {
        let src = Rect::new(0,0,IMAGE_WIDTH,IMAGE_HEIGHT);
        let dest = Rect::new(self.x - game.cam_x + (SCREEN_WIDTH / 2), self.y - game.cam_y + (SCREEN_HEIGHT / 2), OUTPUT_WIDTH, OUTPUT_HEIGHT);    
        let center = Point::new( (OUTPUT_WIDTH/2) as i32, (OUTPUT_HEIGHT) as i32);
        let texture = tex_man.load(&self.texture_path)?;
        game.canvas.copy_ex(
            &texture, // Texture object
            src,      // source rect
            dest,     // destination rect
            0.0,      // angle (degrees)
            center,   // center
            false,    // flip horizontal
            false     // flip vertical
        )?;


        Ok(())
    }

}
