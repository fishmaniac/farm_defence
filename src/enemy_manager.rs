use std::collections::VecDeque;
use sdl2::video::WindowContext;

use crate::constants;
use crate::constants::MAX_WIDTH;
use crate::level_manager;
use crate::level_manager::LevelTile;
use crate::level_manager::TileData;
use crate::game_manager;
use crate::texture_manager;
use crate::player_manager;

pub struct Enemy {
    pub visited: Vec<Vec<bool>>,
    pub queue: VecDeque<(usize, usize)>,
    pub neighbors: Vec<(isize, isize)>,
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

    pub fn place_enemy(&mut self, temp_tile: &level_manager::LevelTile, player: &mut player_manager::PlayerManager, row_index: usize, col_index: usize, row_max: usize, col_max: usize, i: usize) {
        println!("PLACING ENEMY~ X: {}, Y: {}", col_index, row_index);
        match temp_tile.tile_data {
            TileData::Goblin => {
                let mut enemy_tile = self::Enemy {
                    visited: vec![vec![false; col_max]; row_max],
                    queue: VecDeque::new(),
                    neighbors: Vec::new(),
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string(),
                    //CHANGE TEXTURE
                };
                enemy_tile.visited[row_index][col_index] = true;
                enemy_tile.queue.push_back((row_index, col_index));
                self.enemy_vec.push(enemy_tile);
                /*                 println!("GOBLIN PUSHED"); */
            },
            _=> {
                let mut enemy_tile = self::Enemy {
                    visited: vec![vec![false; col_max]; row_max],
                    queue: VecDeque::new(),
                    neighbors: Vec::new(),
                    attack_speed: 5,
                    attack_damage: 5,
                    row_index,
                    col_index,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                enemy_tile.visited[row_index][col_index] = true;
                enemy_tile.queue.push_back((row_index, col_index));
                self.enemy_vec.push(enemy_tile);
            }
        }
    }


    
    pub fn perform_a_star_iteration(enemy: &mut Enemy, target_row: usize, target_col: usize, map: &[Vec<bool>]) {
        let rows = map.len();
        let cols = map[0].len();

        // Initialize the visited and queue data structures
        enemy.visited = vec![vec![false; cols]; rows];
        enemy.queue.clear();

        // Define the neighbors' offsets (up, down, left, right)
        enemy.neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        // Mark the current position as visited and enqueue it
        enemy.visited[enemy.row_index][enemy.col_index] = true;
        enemy.queue.push_back((enemy.row_index, enemy.col_index));

        while let Some((current_row, current_col)) = enemy.queue.pop_front() {
            // Check if the current position is the target
            if current_row == target_row && current_col == target_col {
                // Target reached, perform the movement or attack logic here
                // ...
                return; // Exit the A* algorithm
            }

            // Process the neighbors of the current position
            for &(delta_row, delta_col) in &enemy.neighbors {
                let new_row = current_row as isize + delta_row;
                let new_col = current_col as isize + delta_col;

                // Check if the new position is within the map boundaries and not visited
                if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize &&
                !enemy.visited[new_row as usize][new_col as usize] && map[new_row as usize][new_col as usize] {
                    // Mark the new position as visited and enqueue it
                    enemy.visited[new_row as usize][new_col as usize] = true;
                    enemy.queue.push_back((new_row as usize, new_col as usize));
                }
            }
        }

        // No valid path found
        // Perform alternative action or return an error
        // ...
    }

    fn get_neighbors(graph: &[Vec<LevelTile>], row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let num_rows = graph.len();
        let num_cols = graph[0].len();

        fn is_walkable(graph: &[Vec<LevelTile>], row: usize, col: usize) -> bool {
            match graph[row][col].tile_data {
                TileData::None => graph[row][col].tile_type == constants::TILE_TYPE_GRASS,
                _ => false,
            }
        }

        // Upper neighbor
        if row > 0 && is_walkable(graph, row - 1, col) {
            neighbors.push((row - 1, col));
        }
        // Lower neighbor
        if row < num_rows - 1 && is_walkable(graph, row + 1, col) {
            neighbors.push((row + 1, col));
        }
        // Left neighbor
        if col > 0 && is_walkable(graph, row, col - 1) {
            neighbors.push((row, col - 1));
        }
        // Right neighbor
        if col < num_cols - 1 && is_walkable(graph, row, col + 1) {
            neighbors.push((row, col + 1));
        }

        neighbors
    }

}

