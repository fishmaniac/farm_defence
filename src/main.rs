pub mod constants;
pub mod game_manager;
pub mod texture_manager;
pub mod event_manager;
pub mod player_manager;
pub mod level_manager;
pub mod tower_manager;
pub mod enemy_manager;
pub mod button_manager;

fn game_loop (
    game: &mut game_manager::GameManager, 
    tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
    events: &mut event_manager::EventManager, 
    player: &mut player_manager::PlayerManager,
    level: &mut level_manager::LevelManager,
    towers: &mut tower_manager::TowerManager,
    enemies: &mut enemy_manager::EnemyManager,
    seed_buttons: &mut button_manager::ButtonManager,
    build_buttons: &mut button_manager::ButtonManager,
) {
    let mut fps: u32 = 0;
    let mut frame_count: u32 = 0;
    let mut last_frame_time = std::time::Instant::now();

    while !game.quit {
        frame_count += 1;
        let elapsed_time = last_frame_time.elapsed();

        if elapsed_time >= std::time::Duration::from_secs(1) {
            // Calculate FPS every second
            fps = frame_count;
            frame_count = 0;
            last_frame_time = std::time::Instant::now();
        }
        if frame_count % 16 == 0 {
            println!("FPS: {} FRAME COUNT: {} ELAPSED: {:?}", fps, frame_count, elapsed_time);
        }

        game.prepare_background();
        events.do_event(game, seed_buttons, build_buttons);
        game.update_game(tex_man, player, level, towers, enemies, seed_buttons, build_buttons);
        game.canvas.present();

        // if towers.tower_vec.len() > 0 {
        //     println!("|| GAME || CAM_X: {}, CAM_Y: {} || PLAYER || X: {}, Y: {}, rectX: {}, rectY: {} || TOWER VEC POS || COL: {} ROW: {}", game.cam_x, game.cam_y, player.x, player.y, player.rect.x(), player.rect.y(), towers.tower_vec[0].col_index, towers.tower_vec[0].row_index);
        // }
        //
        /* ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); */
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut game = game_manager::GameManager::new(&sdl_context);
    let texture_creator = game.canvas.texture_creator();
    let mut tex_man = texture_manager::TextureManager::new(&texture_creator);
    let mut events = event_manager::EventManager::new(&sdl_context);
    let mut player = player_manager::PlayerManager::new();
    let mut level = level_manager::LevelManager::new();
    let mut towers = tower_manager::TowerManager::new();
    let mut enemies = enemy_manager::EnemyManager::new();

    let mut seed_buttons = button_manager::ButtonManager::new(constants::SEED_BUTTON_AMT, button_manager::ButtonType::Seed, &player);
    let mut build_buttons = button_manager::ButtonManager::new(constants::BUILD_BUTTON_AMT, button_manager::ButtonType::Build, &player);

    level.create_level(); 
    level.read_file("dungeon.txt").unwrap();


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

    game_loop(&mut game, &mut tex_man, &mut events, &mut player, &mut level, &mut towers, &mut enemies, &mut seed_buttons, &mut build_buttons);

    Ok(())
}
