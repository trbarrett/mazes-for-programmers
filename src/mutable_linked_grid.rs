use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};
use rand::prelude::*;
use rand::seq::IteratorRandom;

use super::grid_primitives::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LinkType {
    Open,
    Closed
}

#[derive(Clone)]
pub struct CellLink {
    cell: Weak<RefCell<GridCell>>,
    pub status: LinkType
}

impl CellLink {
    fn new(cell: Weak<RefCell<GridCell>>) -> Self {
        CellLink {
            cell: cell,
            status: LinkType::Closed,
        }
    }
}

pub struct GridCell {
    pub pos: GridPos,
    pub north: Option<CellLink>,
    pub east: Option<CellLink>,
    pub south: Option<CellLink>,
    pub west: Option<CellLink>,
}

impl GridCell {
    pub fn new(grid_pos: GridPos) -> Self {
        GridCell {
            pos: grid_pos,
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }

    fn link_cell(&mut self, dir: Direction, bidi: bool) {
        self.set_link_status(dir, LinkType::Open);
        if bidi {
            if let Some(rel_cell) = self.get_relative_cell(dir) {
                rel_cell.borrow_mut()
                        .set_link_status(dir.reverse_dir(), LinkType::Open)
            }
        }
    }

    fn get_link(&self, dir: Direction) -> &Option<CellLink> {
        match dir {
            Direction::North => &self.north,
            Direction::East  => &self.east,
            Direction::South => &self.south,
            Direction::West  => &self.west,
        }
    }

    fn get_link_mut(&mut self, dir: Direction) -> &mut Option<CellLink> {
        match dir {
            Direction::North => &mut self.north,
            Direction::East  => &mut self.east,
            Direction::South => &mut self.south,
            Direction::West  => &mut self.west,
        }
    }

    fn get_relative_cell(&self, dir: Direction) -> Option<Rc<RefCell<GridCell>>> {
        self.get_link(dir)
            .as_ref()
            .map(|link| link.cell.upgrade())
            .flatten()
    }

    pub fn set_link_status(&mut self, dir : Direction, status: LinkType) {
        self.get_link_mut(dir)
            .iter_mut()
            .for_each(|link| link.status = status);
    }

    pub fn is_open_to(&self, dir : Direction) -> bool {
        self.get_link(dir)
            .as_ref()
            .map(|link| link.status == LinkType::Open)
            .unwrap_or(false)
    }
}

pub struct MutableLinkedGrid {
    column_count: usize,
    row_count: usize,
    // by row and column
    cells: Vec<Rc<RefCell<GridCell>>>
}

impl MutableLinkedGrid {

    pub fn new(row_count: usize, column_count: usize) -> Self {
        // create cells without any links
        let empty_cells: Vec<Rc<RefCell<GridCell>>> =
            (0..row_count).flat_map(|x| {
                (0..column_count).map(move |y| {
                    Rc::new(RefCell::new(
                        GridCell::new(GridPos { row: Row(x), col: Col(y) })))
                })
            }).collect();
        
        let grid = Self {
            column_count: column_count,
            row_count: row_count,
            cells: empty_cells
        };

        grid.initialize_cells()
    }

    fn initialize_cells(self) -> Self {
        // link all the cells references
        self.cells
            .iter()
            .for_each(|cell| {
                let mut c = cell.borrow_mut();
                let north_pos = self.get_relative_cell_pos(c.pos, Direction::North);
                // use downgrade to get a weak refence
                c.north = north_pos.map(|pos|
                    CellLink::new(Rc::downgrade(&self.get_cell(pos))));

                let east_pos = self.get_relative_cell_pos(c.pos, Direction::East);
                c.east = east_pos.map(|pos|
                    CellLink::new(Rc::downgrade(&self.get_cell(pos))));

                let south_pos = self.get_relative_cell_pos(c.pos, Direction::South);
                c.south = south_pos.map(|pos|
                    CellLink::new(Rc::downgrade(&self.get_cell(pos))));

                let west_pos = self.get_relative_cell_pos(c.pos, Direction::West);
                c.west = west_pos.map(|pos|
                    CellLink::new(Rc::downgrade(&self.get_cell(pos))));
            });
        
        self
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

    fn get_relative_cell_pos(&self, pos: GridPos, dir: Direction) -> Option<GridPos> {
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

    fn get_cell(&self, pos: GridPos) -> &Rc<RefCell<GridCell>> {
        self.cells.get(pos.row.0 * self.row_count + pos.col.0).unwrap()
    }

    pub fn run_sidewinder_algorithm(&self) {
        let mut rng = rand::thread_rng();
        let mut run: Vec<GridPos> = Vec::new();
        let mut row: Option<Row> = None;
        self.cells.iter().for_each(|cell| {
            let pos = {
                let c = cell.borrow();
                c.pos
            };

            // TODO: update the row and reset the run if changed
            if Some(pos.row) != row {
                row = Some(pos.row);
            }

            run.push(pos);

            let should_close_out =
                self.at_eastern_boundary(pos)
                || (!self.at_northern_boundary(pos) && rng.gen_range(0, 2) == 0);
            
            if should_close_out {
                let close_out_pos = *run.iter().choose(&mut rng).unwrap();
                run.clear();
                if !self.at_northern_boundary(pos) {
                    let close_out_cell = self.get_cell(close_out_pos);
                    close_out_cell.borrow_mut().link_cell(Direction::North, true);
                }
            } else {
                cell.borrow_mut().link_cell(Direction::East, true);
            }
        });
    }
}

// **************************
// Iter
// **************************

pub struct Iter<'a> {
    cell_iter: Box<dyn Iterator<Item = &'a Rc<RefCell<GridCell>>> + 'a>
}

impl MutableLinkedGrid {
    pub fn iter(&self) -> Iter {
        Iter {
            cell_iter: Box::new(self.cells.iter())
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Ref<'a, GridCell>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cell_iter.next() {
            None => None,
            Some(cell_ref) => Some(cell_ref.borrow()),
        }
    }
}