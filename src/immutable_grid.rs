use im::hashmap::*;
use rand::prelude::*;
use rand::seq::IteratorRandom;

use super::grid_primitives::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct GridCell {
    pub pos: GridPos,
    pub north_open: bool,
    pub east_open: bool,
    pub south_open: bool,
    pub west_open: bool,
}

impl GridCell {
    pub fn new(pos: GridPos) -> Self {
        GridCell {
            pos: pos,
            north_open: false,
            east_open: false,
            south_open: false,
            west_open: false,
        }
    }

    pub fn is_open_to(&self, dir: Direction) -> bool {
        match dir {
            Direction::North => self.north_open,
            Direction::East  => self.east_open,
            Direction::South => self.south_open,
            Direction::West  => self.west_open,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ImmutableGrid {
    column_count: usize,
    row_count: usize,
    cells: HashMap<GridPos, GridCell>,
}

impl ImmutableGrid {
    pub fn new(column_count : usize, row_count : usize) -> Self {
        let grid_cells: HashMap<GridPos, GridCell> =
            (0..column_count).flat_map(|col| {
                (0..row_count).map(move |row| {
                    let pos = GridPos { col: Col(col), row: Row(row) };
                    let cell = GridCell::new(pos);
                    (pos, cell)
                })
            }).collect();
        
        ImmutableGrid {
            column_count: column_count,
            row_count: row_count,
            cells: grid_cells,
        }
    }

    pub fn at_northern_boundary(&self, pos: GridPos) -> bool {
        pos.row == Row(self.row_count - 1)
    }
    pub fn at_eastern_boundary(&self, pos: GridPos) -> bool {
        pos.col == Col(self.column_count - 1)
    }
    pub fn at_southern_boundary(&self, pos: GridPos) -> bool {
        pos.row == Row(0)
    }
    pub fn at_western_boundary(&self, pos: GridPos) -> bool {
        pos.col == Col(0)
    }

    pub fn get_relative_cell_pos(&self, pos: GridPos, dir: Direction) -> Option<GridPos> {
        if self.at_southern_boundary(pos) && dir == Direction::South {
            return None
        }
    
        if self.at_western_boundary(pos) && dir == Direction::West {
            return None
        }
    
        if self.at_northern_boundary(pos) && dir == Direction::North {
            return None
        }
    
        if self.at_eastern_boundary(pos) && dir == Direction::East {
            return None
        }
    
        let pos =
            match dir {
                Direction::North => GridPos {
                    col: pos.col,
                    row: Row(pos.row.0 + 1)
                },
                Direction::East  => GridPos {
                    col: Col(pos.col.0 + 1),
                    row: pos.row
                },
                Direction::South => GridPos {
                    col: pos.col,
                    row: Row(pos.row.0 - 1)
                },
                Direction::West  => GridPos {
                    col: Col(pos.col.0 - 1),
                    row: pos.row
                },
            };
    
        Some(pos)
    }

    fn update_cell(self, pos: GridPos, cell : GridCell) -> Self {
        ImmutableGrid { 
            column_count: self.column_count,
            row_count: self.row_count,
            cells: self.cells.update(pos, cell),
        }
    }

    pub fn get(&self, pos: &GridPos) -> Option<&GridCell> {
        self.cells.get(&pos)
    }

    fn link_single(self, pos: GridPos, dir: Direction) -> Self {
        let cell = *self.get(&pos).unwrap();
        match dir {
            Direction::North =>
                self.update_cell(pos, GridCell { north_open: true, ..cell } ),
            Direction::East =>
                self.update_cell(pos, GridCell { east_open: true,  ..cell } ),
            Direction::South =>
                self.update_cell(pos, GridCell { south_open: true, ..cell } ),
            Direction::West =>
                self.update_cell(pos, GridCell { west_open: true,  ..cell } ),
        }
    }
    
    fn link_cells(self, pos: GridPos, dir: Direction) -> Self {
        let grid = self.link_single(pos, dir);
        let other_cell_pos = grid.get_relative_cell_pos(pos, dir);
        match other_cell_pos {
            None => grid,
            Some(pos) => grid.link_single(pos, dir.reverse_dir()),
        }
    }

    pub fn grid_rows(&self) -> Vec<(usize, Vec<GridPos>)> {
        (0..self.row_count).map(|row| {
            let row_indexes: Vec<GridPos> =
                (0..self.column_count).map(move |col| {
                    GridPos { col: Col(col), row: Row(row) }
                }).collect();
            (row, row_indexes)
        }).collect()
    }

    pub fn positions(&self) -> Vec<GridPos> {
        let mut positions: Vec<GridPos> = self.iter().map(|cell| cell.pos).collect();
        positions.sort();
        positions
    }

    pub fn run_sidewinder_algorithm(self) -> Self {
        let mut rng = rand::thread_rng();
        self.grid_rows().iter().fold(self.clone(), |grid, (_, row)| {
            let mut run: Vec<GridPos> = Vec::new();
            row.iter().fold(grid, |grid, &pos| {
                run.push(pos);
    
                let should_close_out =
                    grid.at_eastern_boundary(pos)
                    || (!grid.at_northern_boundary(pos) && rng.gen_range(0, 2) == 0);
                
                if should_close_out {
                    let close_out_pos = *run.iter().choose(&mut rng).unwrap();
                    run.clear();
                    if !grid.at_northern_boundary(pos) {
                        grid.link_cells(close_out_pos, Direction::North)
                    } else {
                        grid
                    }
                } else {
                    grid.link_cells(pos, Direction::East)
                }
            })    
        })
    }

    //fn binary_tree_algorithm(grid: HashMap<GridPos, Cell>) -> HashMap<GridPos, Cell> {
    //    let mut rng = rand::thread_rng();
    //    grid.keys().fold(grid.clone(), |acc, k| {
    //        let mut neighbors: Vec<Direction> = Vec::new();
    //        if !at_eastern_boundary(k.col) {
    //            neighbors.push(Direction::East);
    //        }

    //        if !at_northern_boundary(k.row) {
    //            neighbors.push(Direction::North);
    //        } 

    //        let dir = neighbors.iter().choose(&mut rng);
    //        match dir {
    //            Some(&dir) => link_cells(acc, *k, dir),
    //            None => acc,
    //        }
    //    })
    //}


}

// **************************
// Iter
// **************************

// TODO - Try to take a reference to the cells rather than a copy?
pub struct Iter {
    cell_iter: Box<dyn Iterator<Item = (GridPos, GridCell)>>
}

impl ImmutableGrid {
    pub fn iter(&self) -> Iter {
        Iter {
            cell_iter: Box::new(self.cells.clone().into_iter())
        }
    }
}

impl Iterator for Iter {
    type Item = GridCell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cell_iter.next() {
            None => None,
            Some((_, cell)) => Some(cell)
        }
    }
}