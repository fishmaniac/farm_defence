use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::WindowContext;
use sdl2::video::GLProfile;

use crate::level_manager;
use crate::player_manager;
use crate::texture_manager;


//~!~FIXME: REQUIRES MATCHING SCREEN_WIDTH & HEIGHT DEFINITION IN PLAYER_MANAGER~!~
const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct GameManager {
    pub quit: bool,
    pub placing: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub cam_x: i32,
    pub cam_y: i32,
    pub canvas: Canvas<Window>,
    pub mouse_point: sdl2::rect::Point,
    pub mouse_button: sdl2::mouse::MouseButton,
    pub movement: Movement,
}

impl GameManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> GameManager {
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();

        // Don't use deprecated OpenGL functions
        gl_attr.set_context_profile(GLProfile::Core);

        // Set the context into debug mode
        gl_attr.set_context_flags().debug().set();

        // Set the OpenGL context version (OpenGL 3.2)
        gl_attr.set_context_version(3, 2);

        // Enable anti-aliasing
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        let window = video_subsystem
            .window("Bedlam Asylum", SCREEN_WIDTH.try_into().unwrap(), SCREEN_HEIGHT.try_into().unwrap())
            .opengl()
            .resizable()
            .fullscreen_desktop()
            .position_centered()
            .build()
            .expect("Failed to initialize window");
        assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        // ... and we're still using OpenGL 3.2
        assert_eq!(gl_attr.context_version(), (3, 2));

        let canvas = window.into_canvas()
            .present_vsync()
            .build()
            .expect("Failed to initialize canvas");

        let game = GameManager {  
            quit: false,
            placing: false,
            up: false,
            down: false,
            left: false,
            right: false,
            cam_x: 0,
            cam_y: 0,
            canvas,
            mouse_point: sdl2::rect::Point::new(0, 0),
            mouse_button: sdl2::mouse::MouseButton::Unknown,
            movement: Movement::None,
        };
        game
    }

    pub fn prepare_background(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(69, 69, 69, 255));
        self.canvas.clear(); 
    }

    pub fn update_game(&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, level: &mut level_manager::LevelManager) {
        player.update_player(self);
        self.update_camera(player);
        level.render_level(self, player, tex_man).unwrap();
        player.render_player(self, tex_man).unwrap();

    }

    fn update_camera(&mut self, player: &mut player_manager::PlayerManager) {
        self.cam_x = player.x;
        self.cam_y = player.y;
    }
}

