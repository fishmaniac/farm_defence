use crate::{level_manager, button_manager, player_manager, event_manager, texture_manager, constants, tower_manager, enemy_manager, gui_manager, projectile_manager, building_manager, pathfinding_manager, upgrade_manager};

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct GameManager {
    pub placed: bool,
    pub is_pathfinding: bool,
    pub build_mode: bool,
    pub seed_mode: bool,
    pub upgrade_mode: bool,
    pub preview_mode: bool,
    pub hovering_button: bool,
    pub current_seed: usize,
    pub current_build: usize,
    pub carrot_amount: u32,
    pub tomato_amount: u32,
    pub gold_amount: u32,
    pub cam_x: i32,
    pub cam_y: i32,
    pub frame_time: u32,
    pub fps: u32,
    pub elapsed_seconds: f64,
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
        // gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        // gl_attr.set_context_flags().debug().set();
        // gl_attr.set_context_version(3, 2);
        // gl_attr.set_multisample_buffers(1);
        // gl_attr.set_multisample_samples(4);

        let window = video_subsystem
            .window("Farm Defense", 1280, 720)
            .opengl()
/*             .resizable() */
/*             .fullscreen_desktop()   */
/*             .fullscreen() */
            .position_centered()
            .build()
            .expect("Failed to initialize window");
        //
        // assert_eq!(gl_attr.context_profile(), sdl2::video::GLProfile::Core);
        // assert_eq!(gl_attr.context_version(), (3, 2));

        let mut canvas = window.into_canvas()
/*             .present_vsync()    */
            .accelerated()
            .build()
            .expect("Failed to initialize canvas");
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let game = GameManager {  
            placed: false,
            is_pathfinding: false,
            seed_mode: false,
            build_mode: false,
            preview_mode: false,
            upgrade_mode: false,
            hovering_button: false,
            current_seed: usize::MAX,
            current_build: usize::MAX,
            carrot_amount: 0,
            tomato_amount: 0,
            gold_amount: 0,
            cam_x: 0,
            cam_y: 0,
            frame_time: 1,
            fps: 1,
            elapsed_seconds: 0.1,
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
        events: &mut event_manager::EventManager,
        player: &mut player_manager::PlayerManager, 
        level: &mut level_manager::LevelManager, 
        towers: &mut tower_manager::TowerManager, 
        buildings: &mut building_manager::BuildingManager,
        enemies: &mut enemy_manager::EnemyManager, 
        projectiles: &mut projectile_manager::ProjectileManager,
        upgrade_manager: &mut upgrade_manager::UpgradeManager,
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
        pathfinding_manager: &mut pathfinding_manager::PathfindingManager,
    ) {
        player.update_player(events, self, level);
        self.update_camera(player);

        buildings.update_buildings(self, events, level, player, towers, enemies, upgrade_manager, gui_manager, seed_buttons, build_buttons, projectiles);
        level_manager::LevelManager::check_attacks(self, events, player, enemies, towers, buildings, projectiles, gui_manager);
        enemies.move_enemies(events, self, level, pathfinding_manager);
        projectiles.check_projectile_hit(self, events, player, enemies);
        self.delete_all_dead(level, enemies, towers, buildings, projectiles, gui_manager);
    }

    pub fn render_game(
        &mut self, 
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        events: &mut event_manager::EventManager,
        player: &mut player_manager::PlayerManager, 
        level: &mut level_manager::LevelManager, 
        towers: &mut tower_manager::TowerManager, 
        buildings: &mut building_manager::BuildingManager,
        enemies: &mut enemy_manager::EnemyManager, 
        projectiles: &mut projectile_manager::ProjectileManager,
        upgrade_manager: &mut upgrade_manager::UpgradeManager,
        gui_manager: &mut gui_manager::GUIManager,
        seed_buttons: &mut button_manager::ButtonManager, 
        build_buttons: &mut button_manager::ButtonManager,
    ) {


        level.render_level(self, tex_man).unwrap();
        enemy_manager::EnemyManager::render_enemies(enemies, self, tex_man, gui_manager).unwrap(); 
        projectile_manager::ProjectileManager::render_projectiles(projectiles, self, tex_man, events).unwrap();
        tower_manager::TowerManager::render_towers(towers, self, tex_man, gui_manager).unwrap();
        buildings.render_buildings(self, tex_man, gui_manager);
        gui_manager.render_preview(self, tex_man);
        player.render_player(events, self, tex_man).unwrap();
        upgrade_manager.render_upgrade_menu(self);
        seed_buttons.render_seed_buttons(player, tex_man, events, self).unwrap();
        build_buttons.render_build_buttons(player, tex_man, events, self).unwrap();
        gui_manager.render_inventory_hud(events, self, tex_man);
        gui_manager.render_messages(self, events, tex_man);
    }
    pub fn delete_all_dead (
        &mut self,
        level: &mut level_manager::LevelManager,
        enemies: &mut enemy_manager::EnemyManager, 
        towers: &mut tower_manager::TowerManager,
        buildings: &mut building_manager::BuildingManager,
        projectiles: &mut projectile_manager::ProjectileManager,
        gui_manager: &mut gui_manager::GUIManager,
    ) {
        for enemy_index in (0..enemies.enemy_vec.len()).rev() {
            let enemy = &mut enemies.enemy_vec[enemy_index];
            if let Some(target) = enemy.current_target {
                if !self.target_vec.contains(&target) {
                    enemy.current_target = None;
                }
            }

            if enemy.health == 0 {
                level.level_vec[enemy.grid_index.0][enemy.grid_index.1].is_occupied = false;
                enemies.enemy_vec.remove(enemy_index);
                if buildings.building_vec.iter().any(|building| building.building_type == building_manager::BuildingType::Base) {                
                    self.gold_amount += 1;
                }
            }

        }
        for tower_index in (0..towers.tower_vec.len()).rev() {
            let tower = &mut towers.tower_vec[tower_index];

            if tower.health == 0 {
                for target_index in (0..self.target_vec.len()).rev() {
                    let target = self.target_vec[target_index];

                    if target == tower.bottom_index {
                        for enemy in &mut enemies.enemy_vec {
                            if enemy.current_target == Some(target) {
                                enemy.current_target = None;
                            }
                        }

                        self.target_vec.remove(target_index);
                    }
                }
                //@@!!REMOVE AND REDO? this is cause of path bug!!@@
                // for enemy_index in (0..enemies.enemy_vec.len()).rev() {
                //     enemies.enemy_vec[enemy_index].found_target = false;
                // }
                level.level_vec[tower.bottom_index.0][tower.bottom_index.1].tile_type = level.level_vec[tower.bottom_index.0][tower.bottom_index.1].original_type;
                level.level_vec[tower.bottom_index.0][tower.bottom_index.1].tile_data = level_manager::TileData::None;
                level.level_vec[tower.bottom_index.0][tower.bottom_index.1].is_occupied = false;
                towers.tower_vec.remove(tower_index);
            }
        }
        for projectile_index in (0..projectiles.projectile_vec.len()).rev() {
            let projectile = &mut projectiles.projectile_vec[projectile_index];
            let do_despawn_projectile: bool = (projectile.hit_target && projectile.time > constants::PROJECTILE_HIT_DESPAWN_DURATION) || projectile.time > constants::PROJECTILE_DESPAWN_DURATION;

            if do_despawn_projectile {
                projectiles.projectile_vec.remove(projectile_index);
            }
        }
        for building_index in (0..buildings.building_vec.len()).rev() {
            let building = &mut buildings.building_vec[building_index];
            if building.health == 0 {
                if building.building_type == building_manager::BuildingType::Base {
                    //CLEAR TARGET VEC & SET PATH TO LOCATION WHERE ENEMIES CAN BE DESTROYED
                    //ALLOW PLAYER TIME TO REBUILD
                    self.gold_amount = 0;
                    buildings.base_created = false;
                    gui_manager.create_message("base destroyed, time to rebuild".to_string(), 256);
                }
                buildings.building_vec.remove(building_index);
            }
        }
    }

}
