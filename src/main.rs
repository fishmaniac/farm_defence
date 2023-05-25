use sdl2::video::WindowContext;

use std::time::Duration;
// use std::path::Path;

pub mod texture_manager;
pub mod game_manager;
pub mod event_manager;
pub mod player_manager;
pub mod level_manager;

// const IMAGE_WIDTH:u32 = 100;
// const IMAGE_HEIGHT:u32 = 100;
// const OUTPUT_WIDTH: u32 = 50;
// const OUTPUT_HEIGHT: u32 = 50;
// const SCREEN_WIDTH: i32 = 800;
// const SCREEN_HEIGHT: i32 = 600;

fn game_loop <'l, T>(
    mut game: &mut game_manager::GameManager, 
    mut events: &mut event_manager::EventManager, 
    mut tex_man: &mut texture_manager::TextureManager<WindowContext>, 
    mut player: &mut player_manager::PlayerManager,
    mut level: &mut level_manager::LevelManager,
) -> Result<(), String> {
    let texture_path:String = "assets/tile1.png".to_string();
    let texture = &tex_man.load(&texture_path);
    let clone = texture.clone();

    while !game.quit {
        game.prepare_background();

        match clone {
            Ok(ref tex) => {
                game.update_game(&mut player, &mut tex_man, &mut level, tex);
            }
            Err(ref err) => {
                // Handle the error case here
                println!("Error: {}", err);
            }
        }
        events.do_keyboard_event(game);
        //add render error handling

        game.canvas.present();


        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut game = game_manager::GameManager::new(&sdl_context);
    let texture_creator = game.canvas.texture_creator();
    let mut tex_man = texture_manager::TextureManager::new(&texture_creator);
    let mut events = event_manager::EventManager::new(&sdl_context);
    let mut player = player_manager::PlayerManager::new();
    let mut level = level_manager::LevelManager::new();

    level.create_level(); 
    level.read_file("level0.txt").unwrap();

    // Load the images before the main loop so we don't try and load during gameplay
    player.texture_path = "assets/gothicvania/Gothic-hero-Files/PNG/gothic-hero-idle.png".to_string();

    // Prepare fonts
    // let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?; 
    // let font_path: &Path = Path::new(&"assets/font/slkscr.ttf");
    // let mut font = ttf_context.load_font(font_path, 128)?;
    // font.set_style(sdl2::ttf::FontStyle::BOLD);

    //Add game loop error handling
    game_loop::<()>(&mut game, &mut events, &mut tex_man, &mut player, &mut level).unwrap();



    Ok(())
}
