pub mod constants;
pub mod game_manager;
pub mod texture_manager;
pub mod event_manager;
pub mod player_manager;
pub mod level_manager;
pub mod tower_manager;
pub mod building_manager;
pub mod enemy_manager;
pub mod projectile_manager;
pub mod gui_manager;
pub mod button_manager;
pub mod menu_manager;
pub mod pathfinding_manager;
pub mod upgrade_manager;
pub mod utilities;

fn save_game (
    game: &mut game_manager::GameManager, 
    tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
    events: &mut event_manager::EventManager, 
    player: &mut player_manager::PlayerManager,
    level: &mut level_manager::LevelManager,
    towers: &mut tower_manager::TowerManager,
    buildings: &mut building_manager::BuildingManager,
    enemies: &mut enemy_manager::EnemyManager,
    projectiles: &mut projectile_manager::ProjectileManager,
    seed_buttons: &mut button_manager::ButtonManager,
    build_buttons: &mut button_manager::ButtonManager,
    health_bars: &mut gui_manager::GUIManager,
) {
    // let save_path = "saves/save.bin";
    // match level.save_to_file(&save_path) {
    //     Ok(()) => println!("Data saved successfully."),
    //     Err(error) => eprintln!("Failed to save data: {}", error),
    // }
}

fn load_game (
    game: &mut game_manager::GameManager, 
    tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
    events: &mut event_manager::EventManager, 
    player: &mut player_manager::PlayerManager,
    level: &mut level_manager::LevelManager,
    towers: &mut tower_manager::TowerManager,
    buildings: &mut building_manager::BuildingManager,
    enemies: &mut enemy_manager::EnemyManager,
    projectiles: &mut projectile_manager::ProjectileManager,
    seed_buttons: &mut button_manager::ButtonManager,
    build_buttons: &mut button_manager::ButtonManager,
    health_bars: &mut gui_manager::GUIManager,
) {
    // let load_path: &str = "saves/save.bin";
    // let load_file = include_bytes!(load_path);

    // match level_manager::LevelManager::load_from_file(&load_path) {
    //     Ok(loaded_level) => {
    //         *level = loaded_level;
    //         println!("Data loaded successfully.");
    //     }
    //     Err(error) => eprintln!("Failed to load data: {}", error),
    // }
}



fn game_loop (
    game: &mut game_manager::GameManager, 
    tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
    events: &mut event_manager::EventManager, 
    player: &mut player_manager::PlayerManager,
    level: &mut level_manager::LevelManager,
    towers: &mut tower_manager::TowerManager,
    buildings: &mut building_manager::BuildingManager,
    enemies: &mut enemy_manager::EnemyManager,
    projectiles: &mut projectile_manager::ProjectileManager,
    upgrade_manager: &mut upgrade_manager::UpgradeManager,
    seed_buttons: &mut button_manager::ButtonManager,
    build_buttons: &mut button_manager::ButtonManager,
    gui_manager: &mut gui_manager::GUIManager,
    menu_manager: &mut menu_manager::MenuManager,
    pathfinding_manager: &mut pathfinding_manager::PathfindingManager,
) -> Result<(), String> {
    let mut frame_count: u32 = 0;
    let mut last_fps_time = std::time::Instant::now();
    events.current_performance_counter = events.timer_subsystem.performance_counter();

    while !events.game_quit {
        events.last_performance_counter = events.current_performance_counter;
        events.current_performance_counter = events.timer_subsystem.performance_counter();
        events.performance_frequency = events.timer_subsystem.performance_frequency();
        events.delta_time = (events.current_performance_counter - events.last_performance_counter) as f64 / events.performance_frequency as f64;

        game.prepare_background();
        events.do_event(game, towers, seed_buttons, build_buttons, gui_manager);
        if !events.menu_quit {
            menu_manager.update_menu(events, game, player);
        }
        else if !events.game_paused {
            game.update_game(events, player, level, towers, buildings, enemies, projectiles, upgrade_manager, gui_manager, seed_buttons, build_buttons, pathfinding_manager);
            game.render_game(tex_man, events, player, level, towers, buildings, enemies, projectiles, upgrade_manager, gui_manager, seed_buttons, build_buttons);


            game.frame_time += 1;
            frame_count += 1;
            if game.frame_time % 16 == 0 {
                let elapsed_fps_time = last_fps_time.elapsed();
                game.elapsed_seconds = elapsed_fps_time.as_secs_f64();
                game.fps = (frame_count as f64 / game.elapsed_seconds) as u32;
/*                 println!("\nFPS: {}\tELAPSED: {:.4}\tFRAME TIME: {}\tPLAYER POS: X: {}\tY: {}\nCARROTS: {}\tTOMATOES: {}\tDELTA TIME: {}\tPATHING: {}\tBUILD_MODE: {}\tSEED_MODE: {}\tUPGRADE_MODE: {}\n", game.fps, game.elapsed_seconds, game.frame_time, player.x, player.y, game.carrot_amount, game.tomato_amount, events.delta_time, game.is_pathfinding, game.build_mode, game.seed_mode, game.upgrade_mode); */
                frame_count = 0;
                last_fps_time = std::time::Instant::now();
            }

            if game.elapsed_seconds < 2.0 {
                game.is_pathfinding = false;
            }
        }
        if events.game_saving {
            println!("SAVING");
            save_game(game, tex_man, events, player, level, towers, buildings, enemies, projectiles, seed_buttons, build_buttons, gui_manager);
            events.game_saving = false;
        }
        else if events.game_loading {
            println!("LOADING");
            load_game(game, tex_man, events, player, level, towers, buildings, enemies, projectiles, seed_buttons, build_buttons, gui_manager);
            events.game_loading = false;
        }
        game.canvas.present();
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3).unwrap();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    let font_path = std::path::Path::new(&constants::FONT_PATH);
    let small_font = ttf_context.load_font(font_path, 16)?;
    let medium_font = ttf_context.load_font(font_path, 48)?;
    let large_font = ttf_context.load_font(font_path, 64)?;

    let mut game = game_manager::GameManager::new(&sdl_context);
    let texture_creator = game.canvas.texture_creator();
    let mut tex_man = texture_manager::TextureManager::new(&texture_creator);
    let mut events = event_manager::EventManager::new(&sdl_context, &mut game);
    let mut player = player_manager::PlayerManager::new(&mut game, &mut events);
    let mut level = level_manager::LevelManager::new();
    let mut towers = tower_manager::TowerManager::new();
    let mut buildings = building_manager::BuildingManager::new();
    let mut enemies = enemy_manager::EnemyManager::new();
    let mut projectiles = projectile_manager::ProjectileManager::new();
    let mut upgrade_manager = upgrade_manager::UpgradeManager::new(&mut game, &small_font);
    let mut seed_buttons = button_manager::ButtonManager::new(constants::SEED_BUTTON_AMT, button_manager::ButtonType::Seed, &player);
    let mut build_buttons = button_manager::ButtonManager::new(constants::BUILD_BUTTON_AMT, button_manager::ButtonType::Build, &player);
    let mut gui_manager = gui_manager::GUIManager::new(&mut game, &small_font);
    gui_manager.create_inventory_hud(&mut game);

    let mut menu_manager = menu_manager::MenuManager::new(&mut game, &small_font, &medium_font, &large_font);
    menu_manager.create_menu(&mut game, &mut events);

    let mut pathfinding_manager = pathfinding_manager::PathfindingManager::new();

    // TODO: music manager here
    sdl2::mixer::open_audio(44100, sdl2::mixer::DEFAULT_FORMAT, 2, 2048)?;
    sdl2::mixer::allocate_channels(2);

    let audio_chunk = sdl2::mixer::Music::from_file("assets/music/song5.mp3")?;

    sdl2::mixer::Music::play(&audio_chunk, -1);
    sdl2::mixer::Music::set_volume(50);

    level.create_level(); 
    level.read_file("farm.txt").unwrap();

    game_loop(&mut game, &mut tex_man, &mut events, &mut player, &mut level, &mut towers, &mut buildings, &mut enemies, &mut projectiles, &mut upgrade_manager, &mut seed_buttons, &mut build_buttons, &mut gui_manager, &mut menu_manager, &mut pathfinding_manager);

    Ok(())
}
