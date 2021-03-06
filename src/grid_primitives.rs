use std::slice::Iter;

// use tuple struct for Row and Col
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Row(pub usize);

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Col(pub usize);

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridPos {
    pub row: Row,
    pub col: Col,
}

impl GridPos {
    pub fn new(row: Row, col: Col) -> Self {
        GridPos { row: row, col: col}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn reverse_dir(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East  => Direction::West,
            Direction::South => Direction::North,
            Direction::West  => Direction::East,
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::South, Direction::East, Direction::West].iter().copied()
    }
}