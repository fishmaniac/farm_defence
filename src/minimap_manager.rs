//do bitmap load
use crate::{constants, projectile_manager, gui_manager, game_manager, building_manager, player_manager, event_manager, texture_manager::TextureManager, tower_manager, enemy_manager, level_manager};


pub struct MinimapManager <'a> {
    minimap_updated: bool,
    texture: Option<sdl2::render::Texture <'a>>,
    texture_path: String,
}

impl<'a> MinimapManager <'a> {
    pub fn new(game: &mut game_manager::GameManager) -> Self {
        let minimap = MinimapManager {
            minimap_updated: false,
            texture: None,
            texture_path: "assets/default-texture.png".to_string(),
        };
        minimap
    }
    pub fn update_minimap(&mut self, 
        events: &mut event_manager::EventManager, 
        level: &mut level_manager::LevelManager,
        tex_man: &mut TextureManager<sdl2::video::WindowContext>
    ) -> Result<(), String> {
        if events.level_updated {
            println!("UPDATING MINIMAP");
            for col_index in 0..level.level_vec.len() {
                for row_index in 0..level.level_vec[col_index].len() {
                    let level_tile = &mut level.level_vec[col_index][row_index];
                    let texture = tex_man.load(&level_tile.texture_path)?;

                }
            }
            events.level_updated = false;
        }
        Ok(())
    }
    pub fn render_minimap(&mut self,
        game: &mut game_manager::GameManager, 
        level: &mut level_manager::LevelManager,
        tex_man: &mut TextureManager<sdl2::video::WindowContext>,
        player: &mut player_manager::PlayerManager,
    ) -> Result<(), String> {
        for col_index in 0..level.level_vec.len() {
            for row_index in 0..level.level_vec[col_index].len() {
                let texture_path = &level.level_vec[col_index][row_index].texture_path;
                let mut rect = level.level_vec[col_index][row_index].rect;

                rect.set_x(col_index as i32);
                rect.set_y(row_index as i32);
                rect.set_width(1);
                rect.set_height(1);

                if level.level_vec[col_index][row_index].rect.has_intersection(player.rect) {
                    game.canvas.set_draw_color(sdl2::pixels::Color::RED);
                    game.canvas.fill_rect(rect);
                } 
                else {
                    let texture = tex_man.load(&texture_path)?;


                    game.canvas.copy_ex(
                        &texture,
                        None,
                        rect,
                        0.0,
                        None,
                        false,
                        false,
                    )?;
                }
            }
        }
        //
        /* **FOR USING TEXTURE AS MINIMAP
        *    TODO:
        *    LOAD TTF FOR TEXTURE
        *    NEED CODE TO SAVE LEVEL TO TTF
        *
        */    
        //         let rect = sdl2::rect::Rect::new(0, 0, 256, 256);
        // /*         println!("Loading minimap texture: {}", self.texture_path); */
        //         let texture = tex_man.load(&self.texture_path)?;
        //
        //         game.canvas.copy_ex(
        //             &texture,
        //             None,
        //             rect,
        //             0.0,
        //             None,
        //             false,
        //             false,
        //         )?;

        Ok(())
    }
}
