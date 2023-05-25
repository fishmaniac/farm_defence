use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::WindowContext;

use crate::player_manager;
use crate::texture_manager;

pub struct GameManager {
    pub quit: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    cam_x: i32,
    cam_y: i32,
    pub canvas: Canvas<Window>,
}

impl GameManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> GameManager {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Bedlam Asylum", 800, 600)
            .resizable()
            .position_centered()
            .build()
            .expect("Failed to initialize window");

        let mut canvas = window.into_canvas()
            .build()
            .expect("Failed to initialize canvas");
        

        let game = GameManager {  
            quit: false,
            up: false,
            down: false,
            left: false,
            right: false,
            cam_x: 0, //CHANGE TO CAM X & CAM Y
            cam_y: 0,
            canvas,
        };
        game
    }

    pub fn prepare_background(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(69, 69, 69, 255));

        self.canvas.present();
        self.canvas.clear();
    }

    pub fn update_game(&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>) {

        self.test_rect();
        player.update_player(self, tex_man);

/*         println!("X: {}, Y: {}", player.x, player.y); */
    }
    fn test_rect(&mut self) {
        let rect = Rect::new(self.cam_x, self.cam_y, 100, 200);
        let color = Color::RGBA(255, 0, 0, 255);
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }
}

