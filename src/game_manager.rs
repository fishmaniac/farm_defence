use crate::{level_manager, button_manager, player_manager, texture_manager, constants, tower_manager, enemy_manager, gui_manager, projectile_manager, building_manager};

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct GameManager {
    pub quit: bool,
    pub paused: bool,
    pub saving: bool,
    pub loading: bool,
    pub placed: bool,
    pub is_pathfinding: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub build_mode: bool,
    pub seed_mode: bool,
    pub preview_mode: bool,
    pub hovering_button: bool,
    pub current_seed: usize,
    pub current_build: usize,
    pub carrot_amount: u32,
    pub tomato_amount: u32,
    pub cam_x: i32,
    pub cam_y: i32,
    pub frame_time: u32,
    pub fps: u32,
    pub elapsed_seconds: f64,
    pub delta_time: std::time::Duration,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub mouse_point: sdl2::rect::Point,
    pub mouse_button: sdl2::mouse::MouseButton,
    pub target_vec: Vec<(usize, usize)>,
    pub base_location: Option<(usize, usize)>,
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
            .window("Farm Defense", constants::SCREEN_WIDTH.try_into().unwrap(), constants::SCREEN_HEIGHT.try_into().unwrap())
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
            paused: false,
            saving: false,
            loading: false,
            placed: false,
            is_pathfinding: false,
            up: false,
            down: false,
            left: false,
            right: false,
            seed_mode: false,
            build_mode: false,
            preview_mode: false,
            hovering_button: false,
            current_seed: usize::MAX,
            current_build: usize::MAX,
            carrot_amount: 0,
            tomato_amount: 0,
            cam_x: 0,
            cam_y: 0,
            frame_time: 1,
            fps: 1,
            elapsed_seconds: 0.1,
            delta_time: std::time::Duration::new(0, 0),
            canvas,
            mouse_point: sdl2::rect::Point::new(0, 0),
            mouse_button: sdl2::mouse::MouseButton::Unknown,
            target_vec: Vec::new(),
            base_location: None,
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
        player: &mut player_manager::PlayerManager, 
        level: &mut level_manager::LevelManager, 
        towers: &mut tower_manager::TowerManager, 
        buildings: &mut building_manager::BuildingManager,
        enemies: &mut enemy_manager::EnemyManager, 
        projectiles: &mut projectile_manager::ProjectileManager,
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
    ) {
        player.update_player(self, level);
        self.update_camera(player);

        for col_index in 0..level.level_vec.len() {
            for row_index in 0..level.level_vec[col_index].len() {
                let temp_tile = &mut level.level_vec[col_index][row_index];

                buildings.update_buildings(self, towers, enemies, gui_manager, seed_buttons, build_buttons, temp_tile, col_index, row_index);      

            }
        }
        level_manager::LevelManager::check_attacks(self, enemies, towers, buildings, projectiles);
        level.delete_all_dead(self, enemies, towers, buildings, projectiles);
    }


    pub fn render_game(
        &mut self, 
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        player: &mut player_manager::PlayerManager, 
        level: &mut level_manager::LevelManager, 
        towers: &mut tower_manager::TowerManager, 
        buildings: &mut building_manager::BuildingManager,
        enemies: &mut enemy_manager::EnemyManager, 
        projectiles: &mut projectile_manager::ProjectileManager,
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
    ) {


        level.render_level(self, tex_man).unwrap();
        enemy_manager::EnemyManager::render_enemies(enemies, self, tex_man, level, gui_manager).unwrap(); 
        projectile_manager::ProjectileManager::render_projectiles(projectiles, self, tex_man).unwrap();
        tower_manager::TowerManager::render_towers(towers, self, tex_man, gui_manager).unwrap();
        buildings.render_buildings(self, tex_man);
        gui_manager.render_preview(self, tex_man);
        player.render_player(self, tex_man).unwrap();
        seed_buttons.render_seed_buttons(player, tex_man, self).unwrap();
        build_buttons.render_build_buttons(player, tex_man, self).unwrap();  
    }
}
