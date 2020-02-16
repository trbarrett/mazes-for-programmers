 // Using a mutable 2d array doesn't work. The internal array needs to be mutable
 // which makes it almost imposible to pass around and deal with correctly.
 // A better option would be a standard 1d array or vector with the a logic
 // division of rows based on a modulo

//#[derive(Default, Debug, Copy, Clone)]
//struct Cell {
//    north_wall_open: bool,
//    east_wall_open: bool,
//    south_wall_open: bool,
//    west_wall_open: bool,
//}
//
//#[derive(Default, Debug, Copy, Clone)]
//struct Pos {
//    x: usize, 
//    y: usize,
//}
//
//#[derive(Debug, Copy, Clone)]
//enum Direction {
//    North,
//    East,
//    South,
//    West
//}

//fn get_relative_cell_pos(grid : &mut [&[Cell]], pos: Pos, dir: Direction) -> Option<Pos> {
//    if pos.x == 0 || pos.y == 0 {
//        return None
//    }
//
//    if pos.x == grid.len() || pos.y == grid[pos.x].len() {
//        return None
//    }
//
//    let pos =
//        match dir {
//            Direction::North => Pos { x: pos.x,     y: pos.y + 1 },
//            Direction::East  => Pos { x: pos.x + 1, y: pos.y     },
//            Direction::South => Pos { x: pos.x,     y: pos.y - 1 },
//            Direction::West  => Pos { x: pos.x - 1, y: pos.y     },
//        };
//
//    Some(pos)
//}
//
//fn reverse_dir(dir: Direction) -> Direction {
//    match dir {
//        Direction::North => Direction::South,
//        Direction::East  => Direction::East,
//        Direction::South => Direction::North,
//        Direction::West  => Direction::West,
//    }
//}
//
//fn link_single(grid : &mut [&[Cell]], pos: Pos, dir: Direction) {
//    match dir {
//        Direction::North =>
//            grid[pos.x][pos.y] = Cell { north_wall_open: false, ..grid[pos.x][pos.y] },
//        Direction::East =>
//            grid[pos.x][pos.y] = Cell { east_wall_open:  false, ..grid[pos.x][pos.y] },
//        Direction::South =>
//            grid[pos.x][pos.y] = Cell { south_wall_open: false, ..grid[pos.x][pos.y] },
//        Direction::West =>
//            grid[pos.x][pos.y] = Cell { west_wall_open:  false, ..grid[pos.x][pos.y] },
//    }
//}
//
//fn link(grid : &mut [&[Cell]], pos: Pos, dir: Direction) {
//    link_single(grid, pos, dir);
//    let other_cell_pos = get_relative_cell_pos(grid, pos, dir);
//    match other_cell_pos {
//        None => (),
//        Some(pos) => link_single(grid, pos, reverse_dir(dir)),
//    };
//}
//
//fn main() {
//    // create a Grid
//    let walled_cell: Cell = Default::default();
//    let mut grid = [[walled_cell; 10]; 10];
//    
//    link(&grid, Pos { x: 5, y: 5 }, Direction::North);