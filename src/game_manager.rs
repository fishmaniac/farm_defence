use sdl2::video::Window;
use sdl2::render::{Canvas, BlendMode};
use sdl2::pixels::Color;
use sdl2::video::{WindowContext, GLProfile};

use crate::button_manager::ButtonType;
use crate::{level_manager, button_manager, player_manager, texture_manager, constants};

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct GameManager {
    pub quit: bool,
    pub build_mode: bool,
    pub seed_mode: bool,
    pub seed_outline_visible: bool,
    pub build_outline_visible: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub current_seed: usize,
    pub current_build: usize,
    pub carrot_amount: i32,
    pub tomato_amount: i32,
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

        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_version(3, 2);
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        let window = video_subsystem
            .window("Farm Defence", constants::SCREEN_WIDTH.try_into().unwrap(), constants::SCREEN_HEIGHT.try_into().unwrap())
            .opengl()
            .resizable()
            .fullscreen_desktop()
            .position_centered()
            .build()
            .expect("Failed to initialize window");

        assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        assert_eq!(gl_attr.context_version(), (3, 2));

        let mut canvas = window.into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .expect("Failed to initialize canvas");
        canvas.set_blend_mode(BlendMode::Blend);

        let game = GameManager {  
            quit: false,
            seed_mode: false,
            build_mode: false,
            seed_outline_visible: true,
            build_outline_visible: true,
            up: false,
            down: false,
            left: false,
            right: false,
            current_seed: 0,
            current_build: 0,
            carrot_amount: 0,
            tomato_amount: 0,
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
        self.canvas.set_draw_color(Color::RGBA(69, 69, 69, 69));
        self.canvas.clear(); 
    }

    fn update_camera(&mut self, player: &mut player_manager::PlayerManager) {
        self.cam_x = player.x;
        self.cam_y = player.y;
    }

    pub fn update_game(&mut self, player: &mut player_manager::PlayerManager, tex_man: &mut texture_manager::TextureManager<WindowContext>, level: &mut level_manager::LevelManager, seed_buttons: &mut button_manager::ButtonManager, build_buttons: &mut button_manager::ButtonManager) {
        player.update_player(self);
        self.update_camera(player);

        if self.seed_outline_visible || self.build_outline_visible {
            seed_buttons.check_for_clicked(ButtonType::Seed);
            build_buttons.check_for_clicked(ButtonType::Build);
        }

        level.render_level(self, player, tex_man, seed_buttons, build_buttons).unwrap();
        player.render_player(self, tex_man).unwrap();
        seed_buttons.render_seed_buttons(player, tex_man, self).unwrap();
        build_buttons.render_build_buttons(player, tex_man, self).unwrap();  
    }
}
