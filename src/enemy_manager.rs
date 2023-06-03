use sdl2::video::WindowContext;

use std::collections::{BinaryHeap, HashMap};
use std::vec;

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
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    priority: usize,
}

// Implement Ord trait for State to define the ordering in the BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the ordering to create a min-heap
        other.priority.cmp(&self.priority)
    }
}

// Implement PartialOrd trait for State to enable comparison
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* #[derive(Debug)] */
pub struct Enemy {
    pub cost_total: f32,
    pub final_path: Option<Vec<(usize, usize)>>,
    pub col_index: usize,
    pub row_index: usize,
    pub movement_speed: i8,
    pub attack_damage: i8,
    pub found_target: bool,
    pub direction: player_manager::Direction,
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
                let temp_enemy = self::Enemy {
                    cost_total: 0.0,
                    final_path: None,
                    movement_speed: 3,
                    attack_damage: 5,
                    found_target: false,
                    row_index,
                    col_index,
                    direction: player_manager::Direction::Down,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_GOBLIN_ENEMY_FRONT.to_string(),
                };
                self.enemy_vec.push(temp_enemy);
            },
            _=> {
                let temp_enemy = self::Enemy {
                    cost_total: 0.0,
                    final_path: None,
                    movement_speed: 5,
                    attack_damage: 5,
                    found_target: false,
                    row_index,
                    col_index,
                    direction: player_manager::Direction::Down,
                    rect: sdl2::rect::Rect::new(temp_tile.rect.x(), temp_tile.rect.y(), constants::TILE_SIZE, constants::TILE_SIZE),
                    texture_path: constants::TEXTURE_DEFAULT.to_string(),
                };
                self.enemy_vec.push(temp_enemy);
            }
        }
    }

    pub fn render_enemies(
        &mut self,
        game: &mut game_manager::GameManager, 
        tex_man: &mut texture_manager::TextureManager<WindowContext>, 
        level: &mut level_manager::LevelManager, 
    ) -> Result<(), String> {
        for enemy_index in 0..self.enemy_vec.len() {
            let col = self.enemy_vec[enemy_index].col_index as i32;
            let row = self.enemy_vec[enemy_index].row_index as i32;

            self.enemy_vec[enemy_index].rect.set_x((constants::TILE_SIZE as i32 * col as i32) - game.cam_x);
            self.enemy_vec[enemy_index].rect.set_y((constants::TILE_SIZE as i32 * row as i32) - game.cam_y);

            let texture = tex_man.load(&self.enemy_vec[enemy_index].texture_path)?;

            game.canvas.copy_ex(
                &texture, // Texture object
                None,      // source rect
                self.enemy_vec[enemy_index].rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;

            //TODO: maybe make this async...
            if self.enemy_vec[enemy_index].final_path.is_none() && !game.target_vec.is_empty() &&  !self.enemy_vec[enemy_index].found_target {
                let random_index = game.frame_time as usize % game.target_vec.len();
                //TODO: COMBINE IFs
                if (self.enemy_vec[enemy_index].col_index, self.enemy_vec[enemy_index].row_index) != game.target_vec[random_index] {
                    Self::astar(&mut self.enemy_vec[enemy_index], game.target_vec[random_index], &mut level.level_vec);
                    //COMMENT TO KEEP TARGETS
                    /* game.targets.remove(random_index); */
                }
            }
            /*                 println!("PATH: {:?}", self.enemy_vec[enemy_index].final_path);  */
            else if !self.enemy_vec[enemy_index].found_target && game.frame_time % self.enemy_vec[enemy_index].movement_speed as u32 == 0 {
                //FIXME?: remove last element after moving
                if let Some(mut path) = self.enemy_vec[enemy_index].final_path.take() {
                    if let Some((col, row)) = path.first() {
                        level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_type = level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].prev_type;
                        level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data = TileData::None;

                        self.enemy_vec[enemy_index].col_index = *col;
                        self.enemy_vec[enemy_index].row_index = *row;

                        level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_type = constants::TILE_TYPE_GOBLIN_TEST;
                        level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data = TileData::Goblin;

                        path.remove(0);
                        self.enemy_vec[enemy_index].final_path = Some(path);
                    }
                    if self.enemy_vec[enemy_index].final_path.is_none() {
                        self.enemy_vec[enemy_index].found_target = true;

                        match level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data {
                            TileData::Goblin => {
                                level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_type = level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].prev_type;

                                level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data = TileData::None;
                                let random_direction = game.frame_time as usize % 4;
                                match random_direction {
                                    0 => self.enemy_vec[enemy_index].col_index -= 1,
                                    1 => self.enemy_vec[enemy_index].col_index += 1,
                                    2 => self.enemy_vec[enemy_index].row_index -= 1,
                                    3 => self.enemy_vec[enemy_index].row_index += 1,
                                    _ => {}
                                }

                                level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_type = constants::TILE_TYPE_GOBLIN_TEST;
                                level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data = TileData::Goblin;
                                println!("OCCUPIED!");
                            },
                            _ => {}
                        }
                    }
                }
            }

        }
        Ok(())
    }

    pub fn astar(enemy: &mut Enemy, goal: (usize, usize), tiles: &[Vec<LevelTile>]) {
        let mut frontier: BinaryHeap<State> = BinaryHeap::new();
        let mut priorities: HashMap<(usize, usize), usize> = HashMap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let initial_state = State {
            position: (enemy.col_index, enemy.row_index),
            priority: heuristic((enemy.col_index, enemy.row_index), goal),
        };
        frontier.push(initial_state);
        priorities.insert((enemy.col_index, enemy.row_index), initial_state.priority);

        while let Some(current_state) = frontier.pop() {
            let current = current_state.position;

            if current == goal {
                let mut path = vec![current];
                let mut current = current;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                enemy.final_path = Some(path);
            }

            let neighbors = get_neighbors(enemy, current, tiles);

            for next in neighbors {
                //1 FOR 4 WAY OR IMPLEMENT COST
                let new_cost = 1;
                let priority = new_cost + heuristic(next, goal);

                if !priorities.contains_key(&next) || priority < priorities[&next] {
                    priorities.insert(next, priority);
                    frontier.push(State {
                        position: next,
                        priority,
                    });
                    came_from.insert(next, current);
                }
            }
        }
        fn heuristic(position: (usize, usize), goal: (usize, usize)) -> usize {
            let (x1, y1) = position;
            let (x2, y2) = goal;

            let dx = (x1 as isize - x2 as isize).abs() as usize;
            let dy = (y1 as isize - y2 as isize).abs() as usize;

            dx + dy
        }
        fn get_neighbors(enemy: &mut Enemy, position: (usize, usize), level_vec: &[Vec<LevelTile>]) -> Vec<(usize, usize)> {
            let (x, y) = position;
            let width = level_vec[0].len();
            let height = level_vec.len();
            let mut neighbors = Vec::new();

            if y > 0 && level_vec[x][y - 1].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x, y - 1));
            }
            if y < height - 1 && level_vec[x][y + 1].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x, y + 1));
            }
            if x > 0 && level_vec[x - 1][y].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x - 1, y));
            }
            if x < width - 1 && level_vec[x + 1][y].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x + 1, y));
            }
            neighbors
        }
    }
}

