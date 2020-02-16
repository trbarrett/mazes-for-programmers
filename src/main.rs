extern crate piston_window;

use piston_window::*;
use im::hashmap::*;

#[derive(Default, Debug, Copy, Clone)]
struct Cell {
    north_open: bool,
    east_open: bool,
    south_open: bool,
    west_open: bool,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    col: usize, 
    row: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

    
const ROWS: usize  = 10;
const COLUMNS: usize  = 10;

const DRAW_PADDING: f64 = 80.0;
const DRAW_CELL_SIZE: f64 = 40.0;
const FULL_DRAW_WIDTH: f64 = DRAW_CELL_SIZE * ROWS as f64;
const FULL_DRAW_HEIGHT: f64 = DRAW_CELL_SIZE * COLUMNS as f64;

//fn initializeGrid() = 


fn main() {
    let grid: HashMap<Pos, Cell> =
        (0..10).flat_map(|x| {
            (0..10).map(move |y| {
                let default_cell : Cell = Default::default();
                (Pos { col: x, row: y }, default_cell)
            })
        }).collect();

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
            //let white = [1.0, 1.0, 1.0, 100.0]; 

            grid.iter().for_each(|(k, v)| {
                let x1 = k.col as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
                let y1 = k.row as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
                let x2 = x1 + DRAW_CELL_SIZE;
                let y2 = y1 + DRAW_CELL_SIZE;

                if !v.north_open {
                    line_from_to(
                        black, 1.0,
                        [x1, y1], [x2, y1],
                        context.transform, graphics);
                }

                if !v.west_open {
                    line_from_to(
                        black, 1.0,
                        [x1, y1], [x1, y2],
                        context.transform, graphics);
                }

                if k.col == COLUMNS - 1 { // draw the final column end
                    line_from_to(
                        black, 1.0,
                        [x2, y1], [x2, y2],
                        context.transform, graphics);
                }

                if k.row == ROWS - 1 { // draw the final row end
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
