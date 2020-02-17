extern crate piston_window;
extern crate rand;

use piston_window::*;
use im::hashmap::*;
use rand::prelude::*;
use rand::seq::IteratorRandom;
use itertools::Itertools;

#[derive(Default, Debug, Copy, Clone)]
struct Cell {
    north_open: bool,
    east_open: bool,
    south_open: bool,
    west_open: bool,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GridPos {
    row: usize,
    col: usize, 
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

const ROWS: usize = 8;
const COLUMNS: usize = 8;

const DRAW_PADDING: f64 = 80.0;
const DRAW_CELL_SIZE: f64 = 40.0;
const FULL_DRAW_WIDTH: f64 = DRAW_CELL_SIZE * COLUMNS as f64;
const FULL_DRAW_HEIGHT: f64 = DRAW_CELL_SIZE * ROWS as f64;


fn at_eastern_boundary(column: usize) -> bool { column == COLUMNS - 1 }
fn at_northern_boundary(row: usize) -> bool { row == ROWS - 1 }
fn at_southern_boundary(row: usize) -> bool { row == 0 }

fn get_relative_cell_pos(pos: GridPos, dir: Direction) -> Option<GridPos> {
    if pos.row == 0 && dir == Direction::South {
        return None
    }

    if pos.col == 0 && dir == Direction::West {
        return None
    }

    if at_northern_boundary(pos.row) && dir == Direction::North {
        return None
    }

    if at_eastern_boundary(pos.col) && dir == Direction::East {
        return None
    }

    let pos =
        match dir {
            Direction::North => GridPos { col: pos.col,     row: pos.row + 1 },
            Direction::East  => GridPos { col: pos.col + 1, row: pos.row     },
            Direction::South => GridPos { col: pos.col,     row: pos.row - 1 },
            Direction::West  => GridPos { col: pos.col - 1, row: pos.row     },
        };

    Some(pos)
}

fn reverse_dir(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::East  => Direction::West,
        Direction::South => Direction::North,
        Direction::West  => Direction::East,
    }
}

fn link_single(grid: HashMap<GridPos, Cell>, pos: GridPos, dir: Direction) -> HashMap<GridPos, Cell> {
    match dir {
        Direction::North =>
            grid.update(pos, Cell { north_open: true, ..*grid.get(&pos).unwrap() } ),
        Direction::East =>
            grid.update(pos, Cell { east_open: true,  ..*grid.get(&pos).unwrap() } ),
        Direction::South =>
            grid.update(pos, Cell { south_open: true, ..*grid.get(&pos).unwrap() } ),
        Direction::West =>
            grid.update(pos, Cell { west_open: true,  ..*grid.get(&pos).unwrap() } ),
    }
}

fn link_cells(grid: HashMap<GridPos, Cell>, pos: GridPos, dir: Direction) -> HashMap<GridPos, Cell> {
    let grid = link_single(grid, pos, dir);
    let other_cell_pos = get_relative_cell_pos(pos, dir);
    match other_cell_pos {
        None => grid,
        Some(pos) => link_single(grid, pos, reverse_dir(dir)),
    }
}

fn binary_tree_algorithm(grid: HashMap<GridPos, Cell>) -> HashMap<GridPos, Cell> {
    let mut rng = rand::thread_rng();
    grid.keys().fold(grid.clone(), |acc, k| {
        let mut neighbors: Vec<Direction> = Vec::new();
        if !at_eastern_boundary(k.col) {
            neighbors.push(Direction::East);
        }

        if !at_northern_boundary(k.row) {
            neighbors.push(Direction::North);
        } 

        let dir = neighbors.iter().choose(&mut rng);
        match dir {
            Some(&dir) => link_cells(acc, *k, dir),
            None => acc,
        }
    })
}

fn grid_rows(grid: &HashMap<GridPos, Cell>) -> Vec<(usize, Vec<GridPos>)> {
    (0..ROWS).map(|row| {
        let row_indexes: Vec<GridPos> =
            (0..COLUMNS).map(move |col| {
                GridPos { col: col, row: row }
            }).collect();
        (row, row_indexes)
    }).collect()

    //let mut rows: Vec<(usize, Vec<GridPos>)> = Vec::new();
    //for (key, group) in &grid.keys().group_by(|pos| pos.row) {
    //   let mut g: Vec<GridPos> = group.map(|&pos| pos).collect();
    //   g.sort_by(|a, b| a.col.cmp(&b.col));
    //   rows.push((key, g.clone()));
    //}
    //rows
}

fn sidewinder_algorithm(grid: HashMap<GridPos, Cell>) -> HashMap<GridPos, Cell> {
    let mut rng = rand::thread_rng();
    grid_rows(&grid).iter().fold(grid.clone(), |acc, (_, row)| {
        let mut run: Vec<GridPos> = Vec::new();
        row.iter().fold(acc, |acc, k| {
            run.push(*k);

            let should_close_out =
                at_eastern_boundary(k.col)
                || (!at_northern_boundary(k.row) && rng.gen_range(0, 2) == 0);
            
            if should_close_out {
                let pos = *run.iter().choose(&mut rng).unwrap();
                run.clear();
                if !at_northern_boundary(k.row) {
                    link_cells(acc, pos, Direction::North)
                } else {
                    acc
                }
            } else {
                link_cells(acc, *k, Direction::East)
            }
        })    
    })
}



fn main() {
    let grid: HashMap<GridPos, Cell> =
        (0..COLUMNS).flat_map(|x| {
            (0..ROWS).map(move |y| {
                let default_cell : Cell = Default::default();
                (GridPos { col: x, row: y }, default_cell)
            })
        }).collect();

    //let grid = binary_tree_algorithm(grid);
    let grid = sidewinder_algorithm(grid);

    let canvas_sie =
        [ FULL_DRAW_WIDTH + DRAW_PADDING * 2f64,
          FULL_DRAW_HEIGHT + DRAW_PADDING * 2f64 ];

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", canvas_sie)
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            let black = [0.0, 0.0, 0.0, 100.0]; 

            // Note: windows draw from top-left downwards, our grid is from bottom-left going upwards
            grid.iter().for_each(|(k, v)| {
                let x1 = k.col as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
                // Note: row 0 should be at the bottom
                let y1 = (ROWS - 1 - k.row) as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
                let x2 = x1 + DRAW_CELL_SIZE;
                let y2 = y1 + DRAW_CELL_SIZE;

                if !v.north_open {
                    // draw top line
                    line_from_to(
                        black, 1.0,
                        [x1, y1], [x2, y1],
                        context.transform, graphics);
                }

                if !v.west_open {
                    // draw left line
                    line_from_to(
                        black, 1.0,
                        [x1, y1], [x1, y2],
                        context.transform, graphics);
                }

                if at_eastern_boundary(k.col) {
                    // draw right line
                    line_from_to(
                        black, 1.0,
                        [x2, y1], [x2, y2],
                        context.transform, graphics);
                }

                if at_southern_boundary(k.row) {
                    // draw bottom line
                    line_from_to(
                        black, 1.0,
                        [x1, y2], [x2, y2],
                        context.transform, graphics);
                }
            });
           
            //clear([1.0; 4], graphics);
            //rectangle([1.0, 0.0, 0.0, 1.0], // red
            //          [0.0, 0.0, 100.0, 100.0],
            //          context.transform,
            //          graphics);
            //line_from_to(
            //    [0.0, 0.0, 0.0, 1.0], //black
            //    2.0,
            //    [200.0, 200.0],
            //    [300.0, 300.0],
            //    context.transform,
            //    graphics);
        });
    }
}
