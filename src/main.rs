pub mod constants;
pub mod game_manager;
pub mod texture_manager;
pub mod event_manager;
pub mod player_manager;
pub mod level_manager;
pub mod tower_manager;
pub mod enemy_manager;
pub mod projectile_manager;
pub mod gui_manager;
pub mod button_manager;


fn game_loop (
    game: &mut game_manager::GameManager, 
    tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
    events: &mut event_manager::EventManager, 
    player: &mut player_manager::PlayerManager,
    level: &mut level_manager::LevelManager,
    towers: &mut tower_manager::TowerManager,
    enemies: &mut enemy_manager::EnemyManager,
    projectiles: &mut projectile_manager::ProjectileManager,
    seed_buttons: &mut button_manager::ButtonManager,
    build_buttons: &mut button_manager::ButtonManager,
    health_bars: &mut gui_manager::GUIManager,
) {
    let mut frame_count: u32 = 0;
    let mut last_fps_time = std::time::Instant::now();


    while !game.quit {
        game.prepare_background();
        events.do_event(game, seed_buttons, build_buttons, towers);
        game.update_game(player, level, towers, enemies, projectiles, health_bars);
        game.render_game(tex_man, player, level, towers, enemies, projectiles, health_bars, seed_buttons, build_buttons);


        // let player_rect = sdl2::rect::Rect::new(player.rect.x(), player.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE);
        // let player_pos_rect = sdl2::rect::Rect::new(constants::SCREEN_WIDTH as i32 / 2, constants::SCREEN_HEIGHT as i32 / 2, constants::TILE_SIZE, constants::TILE_SIZE);
        //
        // game.canvas.set_draw_color(constants::COLOR_OUTLINE);
        // game.canvas.fill_rect(player_pos_rect);

        game.canvas.present();

        game.frame_time += 1;
        frame_count += 1;

        if game.frame_time % 16 == 0 {
            let elapsed_fps_time = last_fps_time.elapsed();
            game.elapsed_seconds = elapsed_fps_time.as_secs_f64();
            game.fps = (frame_count as f64 / game.elapsed_seconds) as u32;
            println!("\nFPS: {}\tELAPSED: {:.4}\tFRAME TIME: {}\tPLAYER POS: X: {}\tY: {}\nCARROTS: {}\tTOMATOES: {}\t", game.fps, game.elapsed_seconds, game.frame_time, player.x, player.y, game.carrot_amount, game.tomato_amount);
            frame_count = 0;
            last_fps_time = std::time::Instant::now();
        }

        if game.elapsed_seconds < 2.0 {
            game.is_pathfinding = false;
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::MP3).unwrap();

    let mut game = game_manager::GameManager::new(&sdl_context);
    let texture_creator = game.canvas.texture_creator();
    let mut tex_man = texture_manager::TextureManager::new(&texture_creator);
    let mut events = event_manager::EventManager::new(&sdl_context);
    let mut player = player_manager::PlayerManager::new();
    let mut level = level_manager::LevelManager::new();
    let mut towers = tower_manager::TowerManager::new();
    let mut enemies = enemy_manager::EnemyManager::new();
    let mut projectiles = projectile_manager::ProjectileManager::new();

    let mut seed_buttons = button_manager::ButtonManager::new(constants::SEED_BUTTON_AMT, button_manager::ButtonType::Seed, &player);
    let mut build_buttons = button_manager::ButtonManager::new(constants::BUILD_BUTTON_AMT, button_manager::ButtonType::Build, &player);
    let mut health_bars = gui_manager::GUIManager::new();


    // music
    sdl2::mixer::open_audio(44100, sdl2::mixer::DEFAULT_FORMAT, 2, 2048)?;
    sdl2::mixer::allocate_channels(2);

    let audio_chunk = sdl2::mixer::Music::from_file("assets/music/song2.mp3")?;

    sdl2::mixer::Music::play(&audio_chunk, -1);
    sdl2::mixer::Music::set_volume(50);



    level.create_level(); 
    level.read_file("farm.txt").unwrap();


    //~!~!~!~TODO: LOAD IMAGES BEFORE LOOP~!~!~!~

    /*     Prepare fonts */
    // let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    // let font_path: &std::path::Path = std::path::Path::new(&"assets/font/slkscr.ttf");
    // let mut font = ttf_context.load_font(font_path, 128)?;
    // let surface = font.render("FPS: ").unwrap();
    // let texture_creator = sdl_context.video()?.texture_creator();
    // let texture = texture_creator.create_texture_from_surface(&surface)?;

    // font.set_style(sdl2::ttf::FontStyle::BOLD);
    //

    //Add game loop error handling
    //
    // std::thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         std::thread::sleep(Duration::from_millis(1));
    //     }
    // });

    game_loop(&mut game, &mut tex_man, &mut events, &mut player, &mut level, &mut towers, &mut enemies, &mut projectiles, &mut seed_buttons, &mut build_buttons, &mut health_bars);

    Ok(())
}
