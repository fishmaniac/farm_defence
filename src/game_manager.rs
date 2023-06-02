use crate::{level_manager, button_manager, player_manager, texture_manager, constants, tower_manager, enemy_manager};

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
    pub carrot_amount: u32,
    pub tomato_amount: u32,
    pub cam_x: i32,
    pub cam_y: i32,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub mouse_point: sdl2::rect::Point,
    pub mouse_button: sdl2::mouse::MouseButton,
    pub movement: Movement,
    pub targets: Vec<(usize, usize)>,
}

impl GameManager {
    pub fn new(sdl_context: &sdl2::Sdl) -> GameManager {
        let video_subsystem = sdl_context.video().unwrap();
        // let gl_attr = video_subsystem.gl_attr();
        //
        // gl_attr.set_context_profile(GLProfile::Core);
        // gl_attr.set_context_flags().debug().set();
        // gl_attr.set_context_version(3, 2);
        // gl_attr.set_multisample_buffers(1);
        // gl_attr.set_multisample_samples(4);

        let window = video_subsystem
            .window("Farm Defence", constants::SCREEN_WIDTH.try_into().unwrap(), constants::SCREEN_HEIGHT.try_into().unwrap())
            .opengl()
            .resizable()
            .fullscreen_desktop()
            .position_centered()
            .build()
            .expect("Failed to initialize window");
        //
        // assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        // assert_eq!(gl_attr.context_version(), (3, 2));

        let mut canvas = window.into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .expect("Failed to initialize canvas");
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

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
            targets: Vec::new(),
        };
        game
    }

    pub fn prepare_background(&mut self) {
        self.canvas.set_draw_color(constants::COLOR_BACKGROUND);
        self.canvas.clear(); 
    }

    fn update_camera(&mut self, player: &mut player_manager::PlayerManager) {
        self.cam_x = player.x;
        self.cam_y = player.y;
    }

    pub fn update_game(
        &mut self, 
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        player: &mut player_manager::PlayerManager, 
        level: &mut level_manager::LevelManager, 
        towers: &mut tower_manager::TowerManager, 
        enemies: &mut enemy_manager::EnemyManager, 
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
    ) {
        player.update_player(self);
        self.update_camera(player);

        if self.seed_outline_visible || self.build_outline_visible {
            seed_buttons.check_for_clicked(button_manager::ButtonType::Seed);
            build_buttons.check_for_clicked(button_manager::ButtonType::Build);
        }

        for col_index in 0..level.level_vec.len() {
            for row_index in 0..level.level_vec[col_index].len() {
                let temp_tile = &mut level.level_vec[col_index][row_index];  
                
                level_manager::LevelManager::render_level(self, player, tex_man, temp_tile, col_index, row_index).unwrap();
                level_manager::LevelManager::update_buildings(self, towers, enemies, seed_buttons, build_buttons, temp_tile, col_index, row_index);
            }
        }
        tower_manager::TowerManager::render_towers(towers, self, tex_man).unwrap();
        enemy_manager::EnemyManager::render_enemies(enemies, self, tex_man, level).unwrap();





        //TODO: REFACTOR TO LOOP
        player.render_player(self, tex_man).unwrap();
        /* println!("|| GAME || CAM_X: {}, CAM_Y: {} || PLAYER || X: {}, Y: {}, rectX: {}, rectY: {}", self.cam_x, self.cam_y, player.x, player.y, player.rect.x(), player.rect.y()); */
        seed_buttons.render_seed_buttons(player, tex_man, self).unwrap();
        build_buttons.render_build_buttons(player, tex_man, self).unwrap();  
    }
}
