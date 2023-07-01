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
use crate::projectile_manager;

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
    pub max_health: u16,
    pub health: u16,
    pub movement_speed: u8,
    pub attack_damage: u8,
    pub attack_radius: i32,
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
                    movement_speed: constants::ENEMY_GOBLIN_SPEED,
                    attack_damage: constants::ENEMY_GOBLIN_DAMAGE,
                    attack_radius: constants::ENEMY_GOBLIN_RADIUS,
                    max_health: constants::ENEMY_GOBLIN_HEALTH,
                    health: constants::ENEMY_GOBLIN_HEALTH,
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
                    movement_speed: 0,
                    attack_damage: 0,
                    attack_radius: 0,
                    max_health: 0,
                    health: 0,
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

            // let enemy: &mut Enemy = &mut self.enemy_vec[enemy_index];
            // let enemy_position: (&mut usize, &mut usize) = (&mut enemy.col_index, &mut enemy.row_index);
            let has_no_target:bool = self.enemy_vec[enemy_index].final_path.is_none() && !game.target_vec.is_empty() &&  !self.enemy_vec[enemy_index].found_target;
            let can_move:bool = !self.enemy_vec[enemy_index].found_target && game.frame_time % self.enemy_vec[enemy_index].movement_speed as u32 == 0;

            //TODO: maybe make this async...
            if has_no_target {
                let random_index = game.frame_time as usize % game.target_vec.len();
                //TODO: COMBINE IFs
                if (self.enemy_vec[enemy_index].col_index, self.enemy_vec[enemy_index].row_index) != game.target_vec[random_index] {
                    Self::astar(&mut self.enemy_vec[enemy_index], game.target_vec[random_index], &mut level.level_vec);
                    //COMMENT TO KEEP TARGETS
                    /* game.targets.remove(random_index); */
                }
            }
            else if can_move {
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
                    // if self.enemy_vec[enemy_index].final_path.is_none() {
                    //     //makes enemies get stuck after tower destroyed..
                    //     /*                         self.enemy_vec[enemy_index].found_target = true; */
                    //
                    //     match level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data {
                    //         TileData::Goblin => {
                    //             level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_type = level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].prev_type;
                    //             level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data = TileData::None;
                    //
                    //             let random_direction = game.frame_time as usize % 4;
                    //             match random_direction {
                    //                 0 => self.enemy_vec[enemy_index].col_index -= 1,
                    //                 1 => self.enemy_vec[enemy_index].col_index += 1,
                    //                 2 => self.enemy_vec[enemy_index].row_index -= 1,
                    //                 3 => self.enemy_vec[enemy_index].row_index += 1,
                    //                 _ => {}
                    //             }
                    //
                    //             level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_type = constants::TILE_TYPE_GOBLIN_TEST;
                    //             level.level_vec[self.enemy_vec[enemy_index].col_index][self.enemy_vec[enemy_index].row_index].tile_data = TileData::Goblin;
                    //             /*                                 println!("OCCUPIED!"); */
                    //         },
                    //         _ => {}
                    //     }
                    // }
                }
            }

        }
        Ok(())
    }

    pub fn astar(enemy: &mut Enemy, target: (usize, usize), level_vec: &[Vec<LevelTile>]) {
        let initial_state = State {
            position: (enemy.col_index, enemy.row_index),
            priority: heuristic((enemy.col_index, enemy.row_index), target),
        };

        let mut frontier: BinaryHeap<State> = [initial_state].into();
        let mut priorities: HashMap<(usize, usize), usize> = HashMap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        priorities.insert((enemy.col_index, enemy.row_index), initial_state.priority);

        while let Some(current_state) = frontier.pop() {
            let current = current_state.position;

            if current == target {
                let mut path = vec![current];
                let mut current = current;
                while let Some(&prev) = came_from.get(&current) {
                    path.push(prev);
                    current = prev;
                }
                path.reverse();
                enemy.final_path = Some(path);
            }

            let neighbors = get_neighbors(enemy, current, level_vec);

            for next in neighbors {
                //1 FOR 4 WAY OR IMPLEMENT COST
                let new_cost = 1;
                let priority = new_cost + heuristic(next, target);

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
            let mut neighbors = Vec::with_capacity(4);

            //Up
            if y > 0 && level_vec[x][y - 1].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x, y - 1));
            }
            //Down
            if y < height - 1 && level_vec[x][y + 1].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x, y + 1));
            }
            //Left
            if x > 0 && level_vec[x - 1][y].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x - 1, y));
            }
            //Right
            if x < width - 1 && level_vec[x + 1][y].tile_type != constants::TILE_TYPE_WALL {
                neighbors.push((x + 1, y));
            }
            neighbors
        }
    }
}

