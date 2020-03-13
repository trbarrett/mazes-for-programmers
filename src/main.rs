extern crate piston_window;
extern crate rand;
extern crate chrono;

use piston_window::*;
use chrono::prelude::*;

pub mod grid_primitives;
pub mod immutable_grid;
pub mod mutable_linked_grid;
pub mod dijkstra;

use grid_primitives::*;
use immutable_grid::*;
use mutable_linked_grid::*;
use dijkstra::*;

const ROWS: usize = 20;
const COLUMNS: usize = 20;

const DRAW_PADDING: f64 = 20.0;
const DRAW_CELL_SIZE: f64 = 25.0;
const FULL_DRAW_WIDTH: f64 = DRAW_CELL_SIZE * COLUMNS as f64;
const FULL_DRAW_HEIGHT: f64 = DRAW_CELL_SIZE * ROWS as f64;

fn render_grid<G, T>(grid: &ImmutableGrid, context: &Context, graphics: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize {
    let black = [0.0, 0.0, 0.0, 100.0]; 

    // Note: windows draw from top-left downwards, our grid is from bottom-left going upwards
    grid.iter().for_each(|cell| {
        let pos = cell.pos;
        let x1 = pos.col.0 as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
        // Note: row 0 should be at the bottom
        let y1 = (ROWS - 1 - pos.row.0) as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
        let x2 = x1 + DRAW_CELL_SIZE;
        let y2 = y1 + DRAW_CELL_SIZE;

        // colour in dijkstra based on distance number

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
}

fn background_color_for(dijkstra: &Dijkstra, max_distance: f32, pos: GridPos) -> Option<types::Color> {
    let distance = dijkstra.distances.get(&pos);
    distance.map( |&d| {
        let intensity = (max_distance - (d as f32)) / max_distance;
        let dark = 255_f32 * intensity;
        let bright = 128_f32 + (127_f32 * intensity);
        [dark/255_f32, bright/255_f32, dark/255_f32, 1.0]
    })
}

fn render_dijkstra<G, T>(
    grid: &ImmutableGrid,
    dijkstra: &Option<Vec<Dijkstra>>,
    start_time: &Option<DateTime<Utc>>,
    context: &Context,
    g: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize {

    if dijkstra.is_none() { return; }

    let start_time = start_time.unwrap();
    let dijkstra_states = dijkstra.as_ref().unwrap();

    let max_distance = dijkstra_states.last().unwrap().max_distance as f32;

    // depending on how long has passed since the start_time display
    // a different dijkstra state
    let now = Utc::now();
    let duration = now - start_time;
    let count = (duration.num_milliseconds() / 10) as usize;

    let dijkstra = 
        if count >= dijkstra_states.len() {
            dijkstra_states.last().unwrap()
        } else {
            &dijkstra_states[count]
        };


    grid.iter().for_each(|cell| {
        let pos = cell.pos;
        let x1 = pos.col.0 as f64 * DRAW_CELL_SIZE + DRAW_PADDING;
        // Note: row 0 should be at the bottom
        let y1 = (ROWS - 1 - pos.row.0) as f64 * DRAW_CELL_SIZE + DRAW_PADDING;

        if let Some(color) = background_color_for(dijkstra, max_distance, pos) {
            let rectangle = Rectangle::new(color);
            let dims = [x1, y1, DRAW_CELL_SIZE, DRAW_CELL_SIZE];
            rectangle.draw(dims, &draw_state::DrawState::default(), context.transform, g);
        }
    });
}

fn main() {
    // Immutable Grid implementation
    // ---------------------------------------
    // 
    let mut grid = ImmutableGrid::new(COLUMNS, ROWS)
                   .run_sidewinder_algorithm();

    // Mutable Linked Cells implementation
    // ---------------------------------------
    //
    //let grid = MutableLinkedGrid::new(ROWS, COLUMNS);
    //grid.run_binary_tree_algorithm();
    //grid.run_sidewinder_algorithm();

    let mut dijkstraStartTime: Option<DateTime<Utc>> = None;
    let mut dijkstra: Option<Vec<Dijkstra>> = None;


    let canvas_sie =
        [ FULL_DRAW_WIDTH + DRAW_PADDING * 2f64,
          FULL_DRAW_HEIGHT + DRAW_PADDING * 2f64 ];

    let mut window: PistonWindow =
        WindowSettings::new("Mazes for Programmers - Chapter 3!", canvas_sie)
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            render_dijkstra(&grid, &dijkstra, &dijkstraStartTime, &context, graphics);
            render_grid(&grid, &context, graphics);
        });

        if let Some(Button::Keyboard(Key::S)) = event.press_args() {
            grid = ImmutableGrid::new(COLUMNS, ROWS)
                   .run_sidewinder_algorithm();
            dijkstra = None;
        }

        if let Some(Button::Keyboard(Key::B)) = event.press_args() {
            grid = ImmutableGrid::new(COLUMNS, ROWS)
                   .run_binary_tree_algorithm();
            dijkstra = None;
        }

        if let Some(Button::Keyboard(Key::D)) = event.press_args() {
            let d = Dijkstra::new(GridPos::new(Row(ROWS/2 - 1), Col(COLUMNS/2 - 1)))
                    .run_to_completion_all(&grid);
            dijkstra = Some(d);
            dijkstraStartTime = Some(Utc::now());
        }

        if let Some(args) = event.update_args() {
            // render djistra // args.dt
        }
    }
}

//fn update(&mut self, args: &UpdateArgs) {
    // Rotate 2 radians per second.
    //self.rotation += 2.0 * args.dt;
   
    // need a way to decide when to
    // a) start drawing dijstra algo
    // b) when to draw each step
    // c) when to draw longest path process

    // args.time_stamp() // is null for loop :(
    // get diff between time_stamp and drawing foobar
//}