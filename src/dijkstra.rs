use rpds::Queue;
use rpds::HashTrieMap;

use super::grid_primitives::*;
use super::immutable_grid::*;

pub struct Dijkstra {
    pub root: GridPos,
    pub distances: HashTrieMap<GridPos, u32>,
    pub frontier: Queue<GridPos>,
    pub max_distance: u32,
}

impl Dijkstra {
    pub fn new(root: GridPos) -> Self {
        Self {
            root: root,
            distances: HashTrieMap::new().insert(root, 0u32),
            frontier: Queue::new().enqueue(root),
            max_distance: 0u32,
        } 
    }

    pub fn run_to_completion(self, grid: &ImmutableGrid) -> Self {
        let mut state = self;
        while let Some(next) = state.step(grid) {
            state = next;
        }
        state
    }

    pub fn run_to_completion_all(self, grid: &ImmutableGrid) -> Vec<Self> {
        let mut states = Vec::new();
        states.push(self);
        while let Some(next) = states.last().unwrap().step(grid) {
            states.push(next);
        }
        states
    }

    // One step will explore the frontier of the next cell in the frontier
    pub fn step(&self, grid: &ImmutableGrid) -> Option<Self> {
        let pos = self.frontier.peek();
        pos.map(|pos| {
            let mut frontier = self.frontier.dequeue().unwrap();
            let cell = grid.get(&pos).unwrap();
            let d = self.distances.get(&pos).unwrap();
            let mut distances = self.distances.clone();
            let mut max_distance = self.max_distance;

            Direction::iter().for_each(|dir| {
                if cell.is_open_to(dir) {
                    let linked_pos = grid.get_relative_cell_pos(cell.pos, dir);
                    if let Some(linked_pos) = linked_pos {
                        if !distances.contains_key(&linked_pos) {
                            distances = distances.insert(linked_pos, d + 1);
                            frontier = frontier.enqueue(linked_pos);
                            if d + 1 > max_distance { max_distance += 1  }
                        }
                    }
                }
            });

            Self {
                root: self.root,
                distances: distances,
                frontier: frontier,
                max_distance: max_distance,
            } 
        })
    }
}


#[cfg(test)]
mod test {
    use super::super::grid_primitives::*;
    use super::super::immutable_grid::*;
    use super::Dijkstra;

    #[test]
    fn should_do_16_steps_in_4x4_grid() {
        let grid = ImmutableGrid::new(4, 4);
        let mut d = Dijkstra::new(GridPos::new(Row(0), Col(0)));
        let mut steps = 0;
        while let Some(next) = d.step(&grid) {
            steps += 1;
            d = next;
        }
        assert_eq!(steps, 16);
    }

    #[test]
    fn should_visit_each_pos_in_2x2_grid() {
        let grid = ImmutableGrid::new(2, 2);
        let d = Dijkstra::new(GridPos::new(Row(0), Col(0)));
        let d = d.run_to_completion(&grid);
        let mut distance_positions: Vec<GridPos> = d.distances.keys().copied().collect();
        distance_positions.sort();
        assert_eq!(distance_positions, grid.positions());
    }

    #[test]
    fn should_visit_each_pos_in_4x4_grid() {
        let grid = ImmutableGrid::new(2, 2);
        let d = Dijkstra::new(GridPos::new(Row(0), Col(0)));
        let d = d.run_to_completion(&grid);
        let mut distance_positions: Vec<GridPos> = d.distances.keys().copied().collect();
        distance_positions.sort();
        assert_eq!(distance_positions, grid.positions());
    }

    #[test]
    fn should_visit_each_pos_in_4x8_grid() {
        let grid = ImmutableGrid::new(4, 8);
        let d = Dijkstra::new(GridPos::new(Row(0), Col(0)));
        let d = d.run_to_completion(&grid);
        let mut distance_positions: Vec<GridPos> = d.distances.keys().copied().collect();
        distance_positions.sort();
        assert_eq!(distance_positions, grid.positions());
    }

    #[test]
    fn should_visit_each_pos_in_8x30_grid() {
        let grid = ImmutableGrid::new(8, 30);
        let d = Dijkstra::new(GridPos::new(Row(0), Col(0)));
        let d = d.run_to_completion(&grid);
        let mut distance_positions: Vec<GridPos> = d.distances.keys().copied().collect();
        distance_positions.sort();
        assert_eq!(distance_positions, grid.positions());
    }

    #[test]
    fn should_have_all_distances_at_least_up_to_6() {
        let grid = ImmutableGrid::new(4, 4);
        let d = Dijkstra::new(GridPos::new(Row(0), Col(0)));
        let d = d.run_to_completion(&grid);
        for x in 0u32..7u32 {
            assert!(d.distances.values().any(|&v| v == x));
        }
    }

}