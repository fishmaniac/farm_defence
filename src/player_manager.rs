use crate::constants;
use crate::level_manager;
use crate::texture_manager;
use crate::game_manager;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    None,
}

pub struct PlayerManager {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub colliding: bool, 
    pub x: i32,
    pub y: i32,
    pub texture_path: String,
    pub rect: sdl2::rect::Rect,
    pub direction: Direction,
    pub menu_selection: u8,
}

impl PlayerManager {
    pub fn new(game: &mut game_manager::GameManager) -> PlayerManager {
        let player = PlayerManager {
            up: false,
            down: false,
            left: false,
            right: false,
            colliding: false, 
            x: 0,
            y: 0,
            texture_path: "".to_string(),
            rect: sdl2::rect::Rect::new(game.screen_size.0 / 2, game.screen_size.1 / 2, constants::OUTPUT_WIDTH as u32, constants::OUTPUT_HEIGHT as u32),
            direction: Direction::Up,
            menu_selection: 0,
        };
        player
    }

    pub fn update_player(&mut self, 
        game: &mut game_manager::GameManager,
        level: &mut level_manager::LevelManager,
    ) {
        let mut new_x: i32 = self.x;
        let mut new_y: i32 = self.y; 

        if game.up {
            new_y -= constants::PLAYER_SPEED as i32;
            if game.left && !game.right {
                self.direction = Direction::UpLeft;
            }
            else if game.right && !game.left {
                self.direction = Direction::UpRight;
            }
            else {
                self.direction = Direction::Up;
            }
        } 
        if game.down {
            new_y += constants::PLAYER_SPEED as i32;
            if game.left && !game.right {
                self.direction = Direction::DownLeft;
            }
            else if game.right && !game.left {
                self.direction = Direction::DownRight;
            }
            else {
                self.direction = Direction::Down;
            }
        }
        if game.left {
            new_x -= constants::PLAYER_SPEED as i32;
            if game.up && !game.down {
                self.direction = Direction::UpLeft;
            }
            else if game.down && !game.up {
                self.direction = Direction::DownLeft;
            }
            else {
                self.direction = Direction::Left;
            }
        }
        if game.right {
            new_x += constants::PLAYER_SPEED as i32;
            if game.up && !game.down {
                self.direction = Direction::UpRight;
            }
            else if game.down && !game.up {
                self.direction = Direction::DownRight;
            }
            else {
                self.direction = Direction::Right;
            }
        }

        if !self.check_collisions(game, (new_x, new_y), level) {
            self.x = new_x;
            self.y = new_y;
        }
    }
    fn check_collisions (&mut self, game: &mut game_manager::GameManager, new_position: (i32, i32), level: &mut level_manager::LevelManager) -> bool {
        let mut colliding = false;
        let tile_size_offset = constants::TILE_SIZE as i32 / 2;
        let new_offset = constants::TILE_SIZE as i32 / 4;
        let centered_new_x = new_position.0 + (game.screen_size.0 / 2);
        let centered_new_y = new_position.1 + (game.screen_size.1 / 2);
        let new_rect = sdl2::rect::Rect::new(centered_new_x - self.x + new_offset, centered_new_y - self.y + new_offset, tile_size_offset as u32, tile_size_offset as u32);
        for col_index in 0..level.level_vec.len() {
            for row_index in 0..level.level_vec[col_index].len() {
                let temp_tile = &mut level.level_vec[col_index][row_index];

                if temp_tile.rect.has_intersection(new_rect) && temp_tile.tile_type == constants::TILE_TYPE_WALL {
                    colliding = true;
                    break;
                }
            }
            if colliding {
                break;
            }
        }
        colliding
    }
    pub fn render_player(
        &mut self, 
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>
    ) -> Result<(), String> {
        if (game.up || game.down || game.left || game.right) && game.frame_time % constants::PLAYER_SPEED as u32 == 0 {
            match self.direction {
                Direction::Up => self.texture_path = constants::TEXTURE_PLAYER_MOVING_BACK.to_string(),
                Direction::Down => self.texture_path = constants::TEXTURE_PLAYER_MOVING_FRONT.to_string(),
                Direction::Left => self.texture_path = constants::TEXTURE_PLAYER_MOVING_LEFT.to_string(),
                Direction::Right => self.texture_path = constants::TEXTURE_PLAYER_MOVING_RIGHT.to_string(),
                _ => {
                    println!("NO PLAYER_MOVING TEXTURE");
                    self.texture_path = constants::TEXTURE_PLAYER_MOVING_FRONT.to_string();
                }
            }
        }
        else {
            match self.direction {
                Direction::Up => self.texture_path = constants::TEXTURE_PLAYER_BACK.to_string(),
                Direction::Down => self.texture_path = constants::TEXTURE_PLAYER_FRONT.to_string(),
                Direction::Left => self.texture_path = constants::TEXTURE_PLAYER_LEFT.to_string(),
                Direction::Right => self.texture_path = constants::TEXTURE_PLAYER_RIGHT.to_string(),
                Direction::UpLeft => self.texture_path = constants::TEXTURE_PLAYER_BACK_LEFT.to_string(),
                Direction::UpRight => self.texture_path = constants::TEXTURE_PLAYER_BACK_RIGHT.to_string(),
                Direction::DownLeft => self.texture_path = constants::TEXTURE_PLAYER_FRONT_LEFT.to_string(),
                Direction::DownRight => self.texture_path = constants::TEXTURE_PLAYER_FRONT_RIGHT.to_string(),
                _ => {
                    println!("NO PLAYER TEXTURE");
                    self.texture_path = constants::TEXTURE_PLAYER_FRONT.to_string();
                }
            }
        }

        let texture = tex_man.load(&self.texture_path)?;
        game.canvas.copy_ex(
            &texture, // Texture object
            None,      // source rect
            self.rect,     // destination rect
            0.0,      // angle (degrees)
            None,   // center
            false,    // flip horizontal
            false     // flip vertical
        )?;
        Ok(())
    }
}


