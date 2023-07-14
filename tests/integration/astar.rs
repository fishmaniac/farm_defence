mod tests {
    use super::*;

    #[test]
    fn test_pathfinding() {
        // Set up the necessary data for testing
        let mut enemy = Enemy {
            grid_index: (0, 0),  // Set the initial position of the enemy
            final_path: None,   // Initialize the final path as None
            // Initialize other enemy properties as needed
        };

        let target = (3, 3);  // Set the target coordinates

        let level_vec = vec![
            // Initialize the level vector with appropriate LevelTile values
            // ...
        ];

        // Perform the pathfinding
        astar(&mut enemy, target, &level_vec);

        // Assert the expected outcome or behavior
        assert!(enemy.final_path.is_some());
        // Add more assertions as needed
    }}

pub fn astar(enemy: &mut Enemy, target: (usize, usize), level_vec: &[Vec<LevelTile>]) {
    let initial_state = PathState {
        position: enemy.grid_index,
        priority: heuristic(enemy.grid_index, target),
    };

    let mut frontier: std::collections::BinaryHeap<PathState> = [initial_state].into();
    let mut priorities: std::collections::HashMap<(usize, usize), usize> = std::collections::HashMap::new();
    let mut came_from: std::collections::HashMap<(usize, usize), (usize, usize)> = std::collections::HashMap::new();

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
        }

        let neighbors = get_neighbors(current, level_vec);

        for next in neighbors {
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
}
