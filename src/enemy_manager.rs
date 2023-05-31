use sdl2::video::WindowContext;

use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

use crate::constants;
use crate::player_manager;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::LevelTile;
use crate::level_manager::TileData;
use crate::texture_manager;
use crate::enemy_manager;

/* #[derive(Debug, PartialEq, Eq, Hash, Clone)] */
pub struct Enemy {
    pub open_set: BinaryHeap<(usize, (usize, usize))>,
    pub open_set_set: HashSet<(usize, usize)>,
    pub came_from: Vec<Vec<(usize, usize)>>,
    pub g_score: Vec<Vec<usize>>,
    pub f_score: Vec<Vec<usize>>,
    pub row_index: usize,
    pub col_index: usize,
    pub attack_speed: i8,
    pub attack_damage: i8,
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
}

pub struct EnemyManager {
    pub enemy_vec: Vec<Enemy>,
}

impl EnemyManager {
    pub fn new () -> EnemyManager {
        let enemies = EnemyManager {
            enemy_vec: Vec::new(),
        };
        enemies
    }

    pub fn place_enemy(
        &mut self, 
        temp_tile: &level_manager::LevelTile, 
        col_index: usize,
        row_index: usize,
    ) {
        println!("PLACING ENEMY~ X: {}, Y: {}", col_index, row_index);
        match temp_tile.tile_data {
            TileData::Goblin => {
                let enemy_tile = self::Enemy {
                    open_set: BinaryHeap::new(),
                    open_set_set: HashSet::new(),
                    came_from: vec![vec![(usize::MAX, usize::MAX); constants::MAX_WIDTH as usize]; constants::MAX_HEIGHT as usize],
                    g_score: vec![vec![usize::MAX; constants::MAX_WIDTH as usize]; constants::MAX_HEIGHT as usize as usize],
                    f_score: vec![vec![usize::MAX; constants::MAX_WIDTH as usize]; constants::MAX_HEIGHT as usize as usize],
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string(),
                };
                /*                 println!("PATH: {:?}", self.astar((col_index, row_index), (10, 30), &mut level.level_vec));   */
                self.enemy_vec.push(enemy_tile);
            },
            _=> {
                let enemy_tile = self::Enemy {
                    open_set: BinaryHeap::new(),
                    open_set_set: HashSet::new(),
                    came_from: vec![vec![(usize::MAX, usize::MAX); constants::MAX_WIDTH as usize]; constants::MAX_HEIGHT as usize],
                    g_score: vec![vec![usize::MAX; constants::MAX_WIDTH as usize]; constants::MAX_HEIGHT as usize as usize],
                    f_score: vec![vec![usize::MAX; constants::MAX_WIDTH as usize]; constants::MAX_HEIGHT as usize as usize],
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                self.enemy_vec.push(enemy_tile);
            }
        }
    }

    pub fn render_enemies(
        &mut self,
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<WindowContext>, 
        // temp_tile: &mut self::LevelTile, 
        // col_index: usize,
        // row_index: usize,
    ) -> Result<(), String> {
/*         println!("VEC LEN: {}", self.enemy_vec.len()); */
        for enemy_index in 0..self.enemy_vec.len() {
            /*                     match temp_tile.tile_data { */
            // level.level_vec[self.enemy_vec[enemy_index].row_index][self.enemy_vec[enemy_index].col_index].tile_data = TileData::Goblin;
            // level.level_vec[self.enemy_vec[enemy_index].row_index][self.enemy_vec[enemy_index].col_index].texture_path = constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string();
            let col = self.enemy_vec[enemy_index].col_index as i32;
            let row = self.enemy_vec[enemy_index].row_index as i32;

            /*                         TileData::Goblin =>  { */
           /*  if (col_index, row_index) == (self.enemy_vec[enemy_index].row_index, self.enemy_vec[enemy_index].col_index) { */
                self.enemy_vec[enemy_index].rect.set_x((constants::TILE_SIZE as i32 * col as i32) - game.cam_x);
                self.enemy_vec[enemy_index].rect.set_y((constants::TILE_SIZE as i32 * row as i32) - game.cam_y);

                let texture = tex_man.load(constants::TEXTURE_GOBLIN_ENEMY_FRONT)?;

                game.canvas.copy_ex(
                    &texture, // Texture object
                    None,      // source rect
                    self.enemy_vec[enemy_index].rect,     // destination rect
                    0.0,      // angle (degrees)
                    None,   // center
                    false,    // flip horizontal
                    false,     // flip vertical
                )?;
/*             } */

            // if (col_index, row_index) != (10, 30) {
            //     println!("PATH: {:?}", enemies.astar((col_index, row_index), (10, 30), &mut level.level_vec)); 
            //     level.level_vec[col_index][row_index].tile_data = TileData::None;    
            //
            //     /* enemies.bfs(&mut self.level_vec, (col_index, row_index), (10, 30), 0); */
            //     level.level_vec[col_index][row_index].tile_data = TileData::None;
            // }
            //     }
            //     _ => {}
            // }
        }
        Ok(())
    }


    pub fn astar(&mut self, start: (usize, usize), goal: (usize, usize), level_vec: &mut [Vec<LevelTile>]) -> Option<Vec<(usize, usize)>> {
        let height = level_vec.len();
        let width = level_vec[0].len();

        let (start_x, start_y) = start;
        let (goal_x, goal_y) = goal;

        let temp_enemy = &mut self.enemy_vec[0];

        temp_enemy.g_score[start_y][start_x] = 0;
        temp_enemy.f_score[start_y][start_x] = Self::heuristic((start_x, start_y), (goal_x, goal_y));
        temp_enemy.open_set.push((temp_enemy.f_score[start_y][start_x], (start_x, start_y)));
        temp_enemy.open_set_set.insert((start_x, start_y));

        while let Some((_, current)) = temp_enemy.open_set.pop() {
            /* println!("ASTAR: X: {}, Y: {}", current.0, current1); */
            let (current_x, current_y) = current;
            /*             level_vec[current_x][current_y].tile_data = TileData::Goblin; */
            if (current_x, current_y) == (goal_x, goal_y) {
                /*                 println!("GOAL FOUND: X: {} Y: {}", current_x, current_y); */
                // Reconstruct the path from the goal to the start
                let mut path = vec![(goal_x, goal_y)];
                let mut pos = (goal_x, goal_y);

                while pos != (start_x, start_y) {
                    pos = temp_enemy.came_from[pos.1][pos.0];
                    path.push(pos);
                }

                path.reverse();
                return Some(path);
            }


            for neighbor in Self::get_neighbors(current, width, height) {
                let (neighbor_x, neighbor_y) = neighbor;

                let tentative_g_score = temp_enemy.g_score[current_y][current_x] + 1; // Assuming a uniform cost of 1 for adjacent tiles

                if tentative_g_score < temp_enemy.g_score[neighbor_y][neighbor_x] {
                    temp_enemy.came_from[neighbor_y][neighbor_x] = (current_x, current_y);
                    temp_enemy.g_score[neighbor_y][neighbor_x] = tentative_g_score;
                    temp_enemy.f_score[neighbor_y][neighbor_x] = tentative_g_score + Self::heuristic(neighbor, (goal_x, goal_y));

                    if !temp_enemy.open_set_set.contains(&neighbor) {
                        temp_enemy.open_set.push((temp_enemy.f_score[neighbor_y][neighbor_x], neighbor));
                        temp_enemy.open_set_set.insert(neighbor);
                    }
                }
            }
        }
        None
    }    
    fn heuristic(start: (usize, usize), goal: (usize, usize)) -> usize {
        let (start_x, start_y) = start;
        let (goal_x, goal_y) = goal;

        // Manhattan distance
        let dx = (goal_x as isize - start_x as isize).abs() as usize;
        let dy = (goal_y as isize - start_y as isize).abs() as usize;

        dx + dy
    }

    fn get_neighbors(pos: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let mut neighbors = Vec::new();

        if y > 0 {
            neighbors.push((x, y - 1)); // Up
        }
        if y < height - 1 {
            neighbors.push((x, y + 1)); // Down
        }
        if x > 0 {
            neighbors.push((x - 1, y)); // Left
        }
        if x < width - 1 {
            neighbors.push((x + 1, y)); // Right
        }
        neighbors
    }

}

