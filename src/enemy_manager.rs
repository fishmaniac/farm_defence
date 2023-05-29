use std::collections::VecDeque;
use sdl2::video::WindowContext;

use crate::constants;
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

    pub fn place_enemy(&mut self, temp_tile: &level_manager::LevelTile, player: &mut player_manager::PlayerManager, row_index: usize, col_index: usize, row_max: usize, col_max: usize) {
        println!("PLACING ENEMY~ X: {}, Y: {}", col_index, row_index);
        match temp_tile.tile_data {
            TileData::Goblin => {
                let enemy_tile = self::Enemy {
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
                self.enemy_vec.push(enemy_tile);
                /*                 println!("GOBLIN PUSHED"); */
            },
            _=> {
                let enemy_tile = self::Enemy {
                    visited: vec![vec![false; col_max]; row_max],
                    queue: VecDeque::new(),
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

    pub fn bfs(&mut self, graph: &mut [Vec<LevelTile>], start: (usize, usize), target: (usize, usize), i: usize) -> bool {
        self.enemy_vec[i].visited = vec![vec![false; graph[0].len()]; graph.len()];
        self.enemy_vec[i].queue = VecDeque::new();

        self.enemy_vec[i].visited[start.0][start.1] = true;
        // graph[start.0][start.1].tile_data = TileData::None;
        // graph[start.0][start.1].texture_path = constants::TEXTURE_TILE_EMPTY.to_string();

        self.enemy_vec[i].queue.push_back(start);

        while let Some(node) = self.enemy_vec[i].queue.pop_front() {
            graph[node.0][node.1].tile_data = TileData::Goblin;
            println!("||MOVING GOBLIN|| X: {} Y: {}", node.0, node.1);
            if node == target {
                println!("Target node {:?} found!", target);
                return true // Terminate the search if target is found
            }

            let neighbors = Self::get_neighbors(&graph, node.0, node.1);

            for neighbor_coords in neighbors {
                if !self.enemy_vec[i].visited[neighbor_coords.0][neighbor_coords.1] {
                    self.enemy_vec[i].visited[neighbor_coords.0][neighbor_coords.1] = true;
                    self.enemy_vec[i].queue.push_back(neighbor_coords);
                }
            }
            graph[node.0][node.1].tile_data = TileData::None;

        }
        return false
    }

    fn get_neighbors(graph: &[Vec<LevelTile>], row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let num_rows = graph.len();
        let num_cols = graph[0].len();

        // Add neighboring coordinates based on your adjacency rules
        // Example: Add coordinates of all four adjacent tiles

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
