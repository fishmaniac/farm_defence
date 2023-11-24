use crate::constants;
use crate::event_manager;
use crate::level_manager;
use crate::game_manager;
use crate::player_manager;
use crate::enemy_manager;

//this file should be utils.rs

pub fn clamp_to_power_of_two(speed: u16) -> u16 {
    let mut next_power_of_two = 1;
    while next_power_of_two < speed {
        next_power_of_two <<= 1;
    }

    next_power_of_two
}

pub fn clamp_speed(speed: u16, max_speed: u16) -> u16 {
    speed.min(2).max(max_speed);

    speed
}

pub fn tile_not_collidable(tile: &level_manager::LevelTile) -> bool {
    match tile.tile_type {
        constants::TILE_TYPE_WALL => {
            false
        },
        _ => true,
    }
}

pub fn check_player_collisions(
    player: &mut player_manager::PlayerManager,
    game: &mut game_manager::GameManager,
    events: &mut event_manager::EventManager,
    new_position: (i32, i32),
    level: &mut level_manager::LevelManager
) -> bool {
    let mut colliding = false;
    let tile_size_offset = constants::TILE_SIZE as i32 / 2;
    let new_offset = constants::TILE_SIZE as i32 / 4;
    let centered_new_x = new_position.0 + (events.screen_size.0 / 2);
    let centered_new_y = new_position.1 + (events.screen_size.1 / 2);

    //allows for rect collisions but might be slower
    let new_rect = sdl2::rect::Rect::new(
        centered_new_x - player.x + new_offset,
        centered_new_y - player.y + new_offset,
        tile_size_offset as u32,
        tile_size_offset as u32
    );
    for col_index in 0..level.level_vec.len() {
        for row_index in 0..level.level_vec[col_index].len() {
            let temp_tile = &mut level.level_vec[col_index][row_index];

            if temp_tile.tile_type == constants::TILE_TYPE_WALL && temp_tile.rect.has_intersection(new_rect) {
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

pub fn check_enemy_collisions (
    game: &mut game_manager::GameManager,
    events: &mut event_manager::EventManager,
    new_position: (u32, u32),
    level: &mut level_manager::LevelManager
) -> bool {
    let mut colliding = false;
    //shouldnt have to make the box this small
    let tile_size_offset = constants::TILE_SIZE as i32/*  / 16 */;

    //allows for rect collisions but might be slower
    let new_rect = sdl2::rect::Rect::new(
        new_position.0 as i32 - game.cam_x,
        new_position.1 as i32 - game.cam_y,
        tile_size_offset as u32,
        tile_size_offset as u32
    );
    for col_index in 0..level.level_vec.len() {
        for row_index in 0..level.level_vec[col_index].len() {
            let temp_tile = &mut level.level_vec[col_index][row_index];

            if temp_tile.tile_type == constants::TILE_TYPE_WALL && temp_tile.rect.has_intersection(new_rect) {
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

pub fn draw_rect_outline(
        game: &mut game_manager::GameManager,
        rect: sdl2::rect::Rect
    ) {
        game.canvas.set_draw_color(constants::COLOR_OUTLINE);
        game.canvas.draw_line(
            rect.top_left(),
            rect.top_right()
        ).unwrap();
        game.canvas.draw_line(
            rect.bottom_left(),
            rect.bottom_right()
        ).unwrap();
        game.canvas.draw_line(
            rect.top_left(
        ), rect.bottom_left()).unwrap();
        game.canvas.draw_line(
            rect.top_right(),
            rect.bottom_right()
        ).unwrap();
    }
