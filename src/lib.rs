/// Helper structures for creating an input from Stdin, and other structs useful
/// for creating a solution.
use derive_more::{Add, AddAssign, Display, Mul, Sub, SubAssign};
use std::str::FromStr;
use text_io::{scan, try_scan};

pub mod graph;

#[derive(Debug, Clone, Copy, Add, PartialEq, Eq, Sub, AddAssign, Hash, SubAssign, Mul, Display)]
#[display(fmt = "({} {})", _0, _1)]
pub struct Coord(isize, isize);

#[derive(Debug, Clone, Display, Copy, PartialEq, Eq)]
pub enum Color {
    Clear,
    Red,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "R" => Ok(Color::Red),
            "B" => Ok(Color::Blue),
            _ => panic!("Invalid Color"),
        }
    }
}

#[derive(Debug, Clone, Display, PartialEq, Eq, Copy)]
pub enum Circle {
    Target,
    True,
    False,
}

use Circle::*;

impl FromStr for Circle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "C" => Ok(Circle::True),
            "N" => Ok(Circle::False),
            _ => panic!("Invalid Circle"),
        }
    }
}

use Color::*;

#[derive(Debug, Clone, Display, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "N" => Ok(Direction::N),
            "E" => Ok(Direction::E),
            "S" => Ok(Direction::S),
            "W" => Ok(Direction::W),
            "NE" => Ok(Direction::NE),
            "SE" => Ok(Direction::SE),
            "SW" => Ok(Direction::SW),
            "NW" => Ok(Direction::NW),
            _ => panic!("Invalid Direction"),
        }
    }
}

use Direction::*;

#[derive(Debug, Clone, Copy)]
pub struct Arrow {
    loc: Coord,
    color: Color,
    circle: Circle,
    direction: Direction,
}

impl Arrow {
    fn new(loc: Coord, color: Color, circle: Circle, direction: Direction) -> Arrow {
        Arrow {
            loc,
            color,
            circle,
            direction,
        }
    }

    fn is_circle(&self) -> bool {
        self.circle == True
    }

    fn is_target(&self) -> bool {
        self.circle == Target
    }
}

impl Default for Arrow {
    fn default() -> Arrow {
        Arrow {
            loc: Coord(0, 0),
            color: Clear,
            circle: Target,
            direction: N,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub grid: Vec<Vec<Arrow>>,
}

impl Input {
    pub fn from_stdin() -> Self {
        let (rows, cols) = read_rows_cols();

        let mut grid = vec![vec![Arrow::default(); cols]; rows];

        for _ in 0..rows * cols - 1 {
            let (row, col, color, circle, direction): (isize, isize, Color, Circle, Direction);
            scan!("{} {} {} {} {}\n", row, col, color, circle, direction);

            let loc = Coord(row - 1, col - 1);

            let arrow = Arrow::new(loc, color, circle, direction);
            grid[row as usize - 1][col as usize - 1] = arrow;
            // println!("{} {} {} {} {}", row, col, color, circle, direction);
        }

        let (fin_row, fin_col): (isize, isize);
        scan!("{} {} X X X\n", fin_row, fin_col);

        let fin_loc = Coord(fin_row - 1, fin_col - 1);
        let fin_arrow = Arrow::new(fin_loc, Clear, Target, N);
        grid[fin_row as usize - 1][fin_col as usize - 1] = fin_arrow;

        Self { grid }
    }

    pub fn from_grid(grid: Vec<Vec<Arrow>>) -> Self {
        Self { grid }
    }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        coord.0 >= 0
            && coord.1 >= 0
            && (coord.0 as usize) < self.grid.len()
            && (coord.1 as usize) < self.grid[0].len()
    }

    pub fn start(&self) -> Coord {
        Coord(0, 0)
    }

    pub fn target(&self) -> Coord {
        Coord(
            self.grid.len() as isize - 1,
            self.grid[0].len() as isize - 1,
        )
    }
}

fn read_rows_cols() -> (usize, usize) {
    let (rows, cols): (usize, usize);
    scan!("{} {}\n", rows, cols);
    (rows, cols)
}
