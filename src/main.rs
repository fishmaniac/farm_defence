use sdl2::video::WindowContext;

use std::time::Duration;
use std::path::Path;

use button_manager::ButtonType;

pub mod constants;
pub mod texture_manager;
pub mod game_manager;
pub mod event_manager;
pub mod player_manager;
pub mod level_manager;
pub mod button_manager;


fn game_loop (
    game: &mut game_manager::GameManager, 
    events: &mut event_manager::EventManager, 
    tex_man: &mut texture_manager::TextureManager<WindowContext>, 
    player: &mut player_manager::PlayerManager,
    level: &mut level_manager::LevelManager,
    seed_buttons: &mut button_manager::ButtonManager,
    build_buttons: &mut button_manager::ButtonManager
) {
    while !game.quit {
        game.prepare_background();
        events.do_event(game, seed_buttons, build_buttons);

        game.update_game(player, tex_man, level, seed_buttons, build_buttons);
        game.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
    let mut seed_buttons = button_manager::ButtonManager::new(constants::SEED_BUTTON_AMT, ButtonType::Seed, &player);
    let mut build_buttons = button_manager::ButtonManager::new(constants::BUILD_BUTTON_AMT, ButtonType::Build, &player);

    level.create_level(); 
    level.read_file("dungeon.txt").unwrap();

    //~!~!~!~FIXME: Load the images before the main loop so we don't try and load during gameplay~!~!~!~

    /*     Prepare fonts */
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    let font_path: &Path = Path::new(&"assets/font/slkscr.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    //Add game loop error handling
    game_loop(&mut game, &mut events, &mut tex_man, &mut player, &mut level, &mut seed_buttons, &mut build_buttons);

    Ok(())
}
