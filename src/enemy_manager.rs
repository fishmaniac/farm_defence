use crate::constants;
use crate::player_manager;
use crate::game_manager;
use crate::level_manager;
use crate::level_manager::LevelTile;
use crate::level_manager::TileData;
use crate::texture_manager;
use crate::projectile_manager;
use crate::tower_manager;

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathState {
    position: (usize, usize),
    priority: usize,
}

// Implement Ord trait for State to define the ordering in the BinaryHeap
impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse the ordering to create a min-heap
        other.priority.cmp(&self.priority)
    }
}

// Implement PartialOrd trait for State to enable comparison
impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Enemy {
    pub cost_total: f32,
    pub final_path: Option<Vec<(usize, usize)>>,
    pub index: (usize, usize),
    pub max_health: u16,
    pub health: u16,
    pub movement_speed: u8,
    pub attack_damage: u8,
    pub attack_radius: u8,
    pub attack_speed: u8,
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
        index: (usize, usize),
    ) {
        match temp_tile.tile_data {
            TileData::Goblin => {
                let temp_enemy = self::Enemy {
                    cost_total: 0.0,
                    final_path: None,
                    movement_speed: constants::ENEMY_GOBLIN_SPEED,
                    attack_damage: constants::ENEMY_GOBLIN_DAMAGE,
                    attack_radius: constants::ENEMY_GOBLIN_RADIUS,
                    attack_speed: constants::ENEMY_GOBLIN_ATTACK_SPEED,
                    max_health: constants::ENEMY_GOBLIN_HEALTH,
                    health: constants::ENEMY_GOBLIN_HEALTH,
                    found_target: false,
                    index,
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
                    attack_speed: 0,
                    max_health: 0,
                    health: 0,
                    found_target: false,
                    index,
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
        tex_man: &mut texture_manager::TextureManager<sdl2::video::WindowContext>, 
        level: &mut level_manager::LevelManager, 
    ) -> Result<(), String> {
        for enemy in &mut self.enemy_vec {
            let pixel_index: (i32, i32) = (enemy.index.0 as i32 * constants::TILE_SIZE as i32, enemy.index.1 as i32 * constants::TILE_SIZE as i32);

            enemy.rect.set_x(pixel_index.0 as i32 - game.cam_x);
            enemy.rect.set_y(pixel_index.1 as i32 - game.cam_y);

            let texture = tex_man.load(&enemy.texture_path)?;

            game.canvas.copy_ex(
                &texture, // texture object
                None,      // source rect
                enemy.rect,     // destination rect
                0.0,      // angle (degrees)
                None,   // center
                false,    // flip horizontal
                false,     // flip vertical
            )?;

            Self::move_enemies(enemy, game, level);
        }
        Ok(())
    }

    fn move_enemies (enemy: &mut Enemy,
        game: &mut game_manager::GameManager,
        level: &mut level_manager::LevelManager, 
    ) {
        let has_no_targets: bool = !game.target_vec.is_empty() && enemy.final_path.is_none() && !enemy.found_target;
        let can_move: bool = !enemy.found_target && game.frame_time % enemy.movement_speed as u32 == 0;
        let enemy_tuple_index = (enemy.index.0 as i32, enemy.index.1 as i32);

        /*         println!("FOUND TARGET BOOL: {:?} {:?}", enemy.found_target, enemy.index); */
        if can_move {
            if let Some(mut path) = enemy.final_path.take() {
                if let Some((col, row)) = path.first() {
                    enemy.index.0 = *col;
                    enemy.index.1 = *row;

                    path.remove(0);
                    enemy.final_path = Some(path);
                }
            }
        }
        else if has_no_targets{
            let random_index = game.frame_time as usize % game.target_vec.len();
            let target = game.target_vec[random_index];
            let target_tuple_index = (target.0 as i32, target.1 as i32);

            if !game.is_pathfinding && !tower_manager::TowerManager::is_within_area(enemy_tuple_index, target_tuple_index, enemy.attack_radius as i32) {
                Self::astar(enemy, target, &level.level_vec);
                game.is_pathfinding = true;
            }
        }
    }

    pub fn astar(enemy: &mut Enemy, target: (usize, usize), level_vec: &[Vec<LevelTile>]) {
        println!("EXECUTING A*");
        let initial_state = PathState {
            position: enemy.index,
            priority: heuristic(enemy.index, target),
        };

        let mut frontier: std::collections::BinaryHeap<PathState> = [initial_state].into();
        let mut priorities: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
        let mut came_from: std::collections::HashMap<(usize, usize), (usize, usize)> = std::collections::HashMap::new();

        priorities.insert(enemy.index, initial_state.priority);

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

            let neighbors = get_neighbors(current, level_vec);

            for next in neighbors {
                //1 FOR 4 WAY OR IMPLEMENT COST
                let new_cost = 1;
                let priority = new_cost + heuristic(next, target);

                if !priorities.contains_key(&next) || priority < priorities[&next] {
                    priorities.insert(next, priority);
                    frontier.push(PathState {
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
        fn get_neighbors(position: (usize, usize), level_vec: &[Vec<LevelTile>]) -> Vec<(usize, usize)> {
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
