use crate::level_manager;
use crate::enemy_manager;
use crate::utilities;
use crate::constants;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PathState {
    position: (usize, usize),
    priority: usize,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //this could be min or max
        self.priority.cmp(&other.priority)
    }
}

// implement partialord trait for state to enable comparison
impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}


pub struct PathfindingManager {
    pub frontier: Option<std::collections::BinaryHeap<PathState>>,
    pub unsearched: Vec<i8>,
}

impl PathfindingManager {
    pub fn new () -> PathfindingManager {
        let pathfinding_manager = PathfindingManager {
            frontier: None,
            unsearched: Vec::new(),
        };
        pathfinding_manager
    }
    pub fn create_frontier(
        &mut self,
        start: (usize, usize),
        level_vec: &[Vec<level_manager::LevelTile>]
    ) {
        let initial_state = PathState {
            position: start,
            priority: Self::heuristic(start, (10, 10)),
        };

        let frontier: std::collections::BinaryHeap<PathState> = [initial_state].into();
        self.frontier = Some(frontier);
    }
    pub fn astar(
        &mut self,
        enemy: &mut enemy_manager::Enemy,
        target: (usize, usize),
        level_vec: &[Vec<level_manager::LevelTile>]
    ) {
        println!("EXECUTING A*"); 
        let initial_state = PathState {
            position: enemy.grid_index,
            priority: Self::heuristic(enemy.grid_index, target),
        };

        let mut frontier: std::collections::BinaryHeap<PathState> 
        = [initial_state].into();
        let mut priorities: std::collections::HashMap<(usize, usize), usize> 
        = std::collections::HashMap::new();
        let mut came_from: std::collections::HashMap<(usize, usize), (usize, usize)> 
        = std::collections::HashMap::new();

        priorities.insert(enemy.grid_index, initial_state.priority);

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
                println!("ASTAR PATH: {:?}", enemy.final_path);
                return
            }

            let neighbors = Self::get_neighbors(current, level_vec);

            for next in neighbors {
                let new_cost = 1;
                let priority = new_cost + Self::heuristic(next, target);

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
    }
    pub fn astar_new(
        &mut self,
        enemy: &mut enemy_manager::Enemy,
        target: (usize, usize),
        level_vec: &[Vec<level_manager::LevelTile>]
    ) {

    }
    fn get_neighbors(start: (usize, usize), level_vec: &[Vec<level_manager::LevelTile>]) -> Vec<(usize, usize)> {
        let (x, y) = start;
        let width = level_vec[0].len();
        let height = level_vec.len();
        let mut neighbors = Vec::with_capacity(8);
        let top_tile = &level_vec[x][y - 1];
        let bottom_tile = &level_vec[x][y + 1]; 
        let left_tile = &level_vec[x - 1][y];
        let right_tile = &level_vec[x + 1][y];
        let top_left_tile = &level_vec[x - 1][y - 1];
        let top_right_tile = &level_vec[x - 1][y + 1];
        let bottom_left_tile = &level_vec[x + 1][y - 1];
        let bottom_right_tile = &level_vec[x + 1][y + 1];

        // Top
        if !top_tile.is_occupied 
        && y > 0 
        && utilities::tile_not_collidable(top_tile) {
            neighbors.push((x, y - 1));
        }
        // Bottom
        if !bottom_tile.is_occupied 
        && y < height - 1
        && utilities::tile_not_collidable(bottom_tile) {
            neighbors.push((x, y + 1));
        }
        // Left
        if !left_tile.is_occupied 
        && x > 0
        && utilities::tile_not_collidable(left_tile) {
            neighbors.push((x - 1, y));
        }
        // Right
        if !right_tile.is_occupied 
        && x < width - 1 
        && utilities::tile_not_collidable(right_tile) {
            neighbors.push((x + 1, y));
        }
        // Top-left
        if !top_left_tile.is_occupied 
        && x > 0 && y > 0 
        && utilities::tile_not_collidable(top_left_tile) {
            neighbors.push((x - 1, y - 1));
        }
        // Top-right
        if !top_right_tile.is_occupied 
        && x > 0 && y < height - 1 
        && utilities::tile_not_collidable(top_right_tile) {
            neighbors.push((x - 1, y + 1));
        }
        // Bottom-left
        if !bottom_left_tile.is_occupied
        && x < width - 1 && y > 0 
        && utilities::tile_not_collidable(bottom_left_tile) {

            neighbors.push((x + 1, y - 1));
        }
        // Bottom-right
        if !bottom_right_tile.is_occupied
        && x < width - 1 && y < height - 1 
        && utilities::tile_not_collidable(bottom_right_tile) {
            neighbors.push((x + 1, y + 1));
        }            
        neighbors
    }

    fn heuristic(start: (usize, usize), target: (usize, usize)) -> usize {
        let (x1, y1) = start;
        let (x2, y2) = target;

        let dx = (x1 as isize - x2 as isize).abs() as usize;
        let dy = (y1 as isize - y2 as isize).abs() as usize;

        dx + dy
    }
}
