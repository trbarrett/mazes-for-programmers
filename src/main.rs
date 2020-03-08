extern crate piston_window;
extern crate rand;

use piston_window::*;

pub mod grid_primitives;
pub mod immutable_grid;
pub mod mutable_linked_grid;

use grid_primitives::*;
use immutable_grid::*;
use mutable_linked_grid::*;

const ROWS: usize = 50;
const COLUMNS: usize = 50;

const DRAW_PADDING: f64 = 20.0;
const DRAW_CELL_SIZE: f64 = 12.0;
const FULL_DRAW_WIDTH: f64 = DRAW_CELL_SIZE * COLUMNS as f64;
const FULL_DRAW_HEIGHT: f64 = DRAW_CELL_SIZE * ROWS as f64;


fn main() {

    // Immutable Grid implementation
    // ---------------------------------------
    // 
    let grid = ImmutableGrid::new(COLUMNS, ROWS);
    //let grid = binary_tree_algorithm(grid);
    let grid = grid.run_sidewinder_algorithm();

    // Mutable Linked Cells implementation
    // ---------------------------------------
    //
    //let grid = MutableLinkedGrid::new(ROWS, COLUMNS);
    //grid.run_binary_tree_algorithm();
    //grid.run_sidewinder_algorithm();

    let canvas_sie =
        [ FULL_DRAW_WIDTH + DRAW_PADDING * 2f64,
          FULL_DRAW_HEIGHT + DRAW_PADDING * 2f64 ];

    let mut window: PistonWindow =
        WindowSettings::new("Mazes for Programmers - Chapter 2!", canvas_sie)
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            let black = [0.0, 0.0, 0.0, 100.0]; 

            // Note: windows draw from top-left downwards, our grid is from bottom-left going upwards
            grid.iter().for_each(|cell| {
                let pos = cell.pos;
                let x1 = pos.col.0 as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
                // Note: row 0 should be at the bottom
                let y1 = (ROWS - 1 - pos.row.0) as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
                let x2 = x1 + DRAW_CELL_SIZE;
                let y2 = y1 + DRAW_CELL_SIZE;

                if !cell.is_open_to(Direction::North) {
                    // draw top line
                    line_from_to(
                        black, 1.0,
                        [x1, y1], [x2, y1],
                        context.transform, graphics);
                }

                if !cell.is_open_to(Direction::West) {
                    // draw left line
                    line_from_to(
                        black, 1.0,
                        [x1, y1], [x1, y2],
                        context.transform, graphics);
                }

                if grid.at_eastern_boundary(pos) {
                    // draw right line
                    line_from_to(
                        black, 1.0,
                        [x2, y1], [x2, y2],
                        context.transform, graphics);
                }

                if grid.at_southern_boundary(pos) {
                    // draw bottom line
                    line_from_to(
                        black, 1.0,
                        [x1, y2], [x2, y2],
                        context.transform, graphics);
                }
            });
        });
    }
}
