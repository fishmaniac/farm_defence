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
    pub row_index: usize,
    pub col_index: usize,
    pub attack_speed: i8,
    pub attack_damage: i8,
    pub rect: sdl2::rect::Rect,
    pub texture_path: String,
}

pub struct EnemyManager {
    pub enemy_vec: Vec<Enemy>,
    pub path_open_list: Vec<Vec<LevelTile>>,
}

impl EnemyManager {
    pub fn new () -> EnemyManager {
        let enemies = EnemyManager {
            enemy_vec: Vec::new(),
            path_open_list: Vec::new(),
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


                // self.enemy_vec[i].visited = vec![vec![false; constants::MAX_HEIGHT as usize]; constants::MAX_WIDTH as usize];
                // self.enemy_vec[i].queue = VecDeque::new();
            }
        }
    }

    pub fn bfs(&mut self, graph: &mut [Vec<LevelTile>], current: (usize, usize), target: (usize, usize), i: usize) -> bool {
        // self.enemy_vec[i].visited = vec![vec![false; graph[0].len()]; graph.len()];
        // self.enemy_vec[i].queue = VecDeque::new();

/*         self.enemy_vec[i].visited[current.0][current.1] = true; */
        // graph[current.0][current.1].tile_data = TileData::None;
        // graph[current.0][current.1].texture_path = constants::TEXTURE_TILE_EMPTY.to_string();

/*         self.enemy_vec[i].queue.push_back(current); */

        if let Some(current) = self.enemy_vec[i].queue.pop_front() {
            graph[current.0][current.1].tile_data = TileData::Goblin;
            println!("||MOVING GOBLIN|| X: {} Y: {}", current.0, current.1);
            if current == target {
                println!("Target node {:?} found!", target);
                return true;  
            }

            let neighbors = Self::get_neighbors(&graph, current.0, current.1);

            for neighbor_coords in neighbors {
                if !self.enemy_vec[i].visited[neighbor_coords.0][neighbor_coords.1] {
                    self.enemy_vec[i].visited[neighbor_coords.0][neighbor_coords.1] = true;
                    self.enemy_vec[i].queue.push_back(neighbor_coords);
                }
            }
            graph[current.0][current.1].tile_data = TileData::None;
        }

        false
    }

    fn get_neighbors(graph: &[Vec<LevelTile>], row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let num_rows = graph.len();
        let num_cols = graph[0].len();

        // Upper neighbor
        if row > 0 {
            neighbors.push((row - 1, col));
        }
        // Lower neighbor
        if row < num_rows - 1 {
            neighbors.push((row + 1, col));
        }
        // Left neighbor
        if col > 0 {
            neighbors.push((row, col - 1));
        }
        // Right neighbor
        if col < num_cols - 1 {
            neighbors.push((row, col + 1));
        }
        neighbors
    }
}