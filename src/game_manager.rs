use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::WindowContext;

use crate::level_manager;
use crate::player_manager;
use crate::texture_manager;


//~!~FIXME: REQUIRES MATCHING SCREEN_WIDTH & HEIGHT DEFINITION IN PLAYER_MANAGER~!~
const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;

pub struct GameManager {
    pub quit: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub cam_x: i32,
    pub cam_y: i32,
    pub canvas: Canvas<Window>,
}

impl GameManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> GameManager {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Bedlam Asylum", SCREEN_WIDTH.try_into().unwrap(), SCREEN_HEIGHT.try_into().unwrap())
            .resizable()
            .fullscreen_desktop()
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
        self.canvas.clear();
    }

    pub fn update_game(&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, level: &mut level_manager::LevelManager) {
        self.test_rect();
        level.render_level(self, player);
        player.update_player(self, tex_man);
        self.update_camera(player);
    }

    fn test_rect(&mut self) {
        let rect = Rect::new(self.cam_x, self.cam_y, 100, 200);
        let color = Color::RGBA(255, 0, 0, 255);
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }
    fn update_camera(&mut self, player: &mut player_manager::PlayerManager) {
        self.cam_x = player.x;
        self.cam_y = player.y;
    }
}

