use crate::level_manager;
use crate::enemy_manager;
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
    pub fn create_frontier(&mut self, start: (usize, usize), level_vec: &[Vec<level_manager::LevelTile>]) {
        let initial_state = PathState {
            position: start,
            priority: Self::heuristic(start, (10, 10)),
        };

        let frontier: std::collections::BinaryHeap<PathState> = [initial_state].into();
        self.frontier = Some(frontier);
    }

    pub fn astar(&mut self, enemy: &mut enemy_manager::Enemy, target: (usize, usize), level_vec: &[Vec<level_manager::LevelTile>]) {
        let mut final_path: Vec<(usize, usize)> = Vec::new();

        let initial_state = PathState {
            position: enemy.grid_index,
            priority: Self::heuristic(enemy.grid_index, target),
        };
        println!("Initial Priotity: {:?}{}", enemy.grid_index, initial_state.priority);

        let mut frontier: std::collections::BinaryHeap<PathState> = [initial_state].into();

        while !frontier.is_empty() {
            let current = frontier.pop();

/*             let neighbors = Self::get_neighbors(current, level_vec); */
        }
    }
    fn get_neighbors(position: (usize, usize), level_vec: &[Vec<level_manager::LevelTile>]) -> Vec<(usize, usize)> {
        let (x, y) = position;
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

        let tile_types_to_avoid = [
            constants::TILE_TYPE_WALL,
        ];

        //Up
        if y > 0 && !tile_types_to_avoid.contains(&top_tile.tile_type) && !top_tile.is_occupied {
            neighbors.push((x, y - 1));
        }
        //Down
        if y < height - 1 && !tile_types_to_avoid.contains(&bottom_tile.tile_type) && !bottom_tile.is_occupied {
            neighbors.push((x, y + 1));
        }
        //Left
        if x > 0 && !tile_types_to_avoid.contains(&left_tile.tile_type) && !left_tile.is_occupied {
            neighbors.push((x - 1, y));
        }
        //Right
        if x < width - 1 && !tile_types_to_avoid.contains(&right_tile.tile_type) && !right_tile.is_occupied {
            neighbors.push((x + 1, y));
        }
        // Top-left
        if x > 0 && y > 0 && !tile_types_to_avoid.contains(&top_left_tile.tile_type) && !top_left_tile.is_occupied {
            neighbors.push((x - 1, y - 1));
        }
        // Top-right
        if x > 0 && y < height - 1 && !tile_types_to_avoid.contains(&top_right_tile.tile_type) && !top_right_tile.is_occupied {
            neighbors.push((x - 1, y + 1));
        }
        // Bottom-left
        if x < width - 1 && y > 0 && !tile_types_to_avoid.contains(&bottom_left_tile.tile_type) && !bottom_left_tile.is_occupied {
            neighbors.push((x + 1, y - 1));
        }
        // Bottom-right
        if x < width - 1 && y < height - 1 && !tile_types_to_avoid.contains(&bottom_right_tile.tile_type) && !bottom_right_tile.is_occupied {
            neighbors.push((x + 1, y + 1));
        }            
        neighbors
    }
    fn heuristic(position: (usize, usize), goal: (usize, usize)) -> usize {
        let (x1, y1) = position;
        let (x2, y2) = goal;

        let dx = (x1 as isize - x2 as isize).abs() as usize;
        let dy = (y1 as isize - y2 as isize).abs() as usize;

        dx + dy
    }
}
