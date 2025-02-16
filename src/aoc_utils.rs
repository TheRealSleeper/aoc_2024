use std::env::args;
use std::ops::{Index, IndexMut};
use std::process::exit;

pub struct Args {
    pub part1: bool,
    pub part2: bool,
    pub verbose: bool,
    pub path: Option<String>,
    pub sample: bool,
}

const HELP: &str = "TODO: Add help text";

impl Args {
    pub fn get() -> Args {
        let mut my_args = Args {
            part1: false,
            part2: false,
            verbose: false,
            path: None,
            sample: false,
        };

        let mut env_args = args().skip(1);

        while let Some(a) = env_args.next() {
            if a.chars().nth(0).unwrap() == '-' {
                if a.chars().nth(1).unwrap() == '-' {
                    match a.chars().skip(2).collect::<String>().as_str() {
                        "part1" => my_args.part1 = true,
                        "part2" => my_args.part2 = true,
                        "verbose" => my_args.verbose = true,
                        "input" => {
                            my_args.path =
                                Some(env_args.next().expect("Input argument missing").to_string())
                        }
                        "sample" => my_args.sample = true,
                        "help" => println!("{}", HELP),
                        _ => {
                            println!("Unrecognized option {}", a);
                            exit(1);
                        }
                    }
                } else {
                    for c in a.chars().skip(1) {
                        match c {
                            '1' => my_args.part1 = true,
                            '2' => my_args.part2 = true,
                            'v' => my_args.verbose = true,
                            'i' => {
                                my_args.path = Some(
                                    env_args.next().expect("Input argument missing").to_string(),
                                )
                            }
                            's' => my_args.sample = true,
                            'h' => println!("{}", HELP),
                            _ => {
                                println!("Unrecognized option {}", a);
                                exit(1);
                            }
                        }
                    }
                }
            } else {
                println!("Invalid option '{}' given", a);
                exit(1);
            }
        }

        my_args
    }
}

/// 2D grid object with easy bounds checking
#[derive(Clone, Debug)]
pub struct Grid<T> {
    rows: usize,
    columns: usize,
    data: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Allows easy traversal of ```Grid<T>```
#[derive(Debug, Clone)]
pub struct Position<'a, T> {
    grid: &'a Grid<T>,
    row: isize,
    column: isize,
    direction: Direction,
}

#[derive(Debug)]
struct PositionMut<'a, T> {
    grid: &'a mut Grid<T>,
    row: isize,
    column: isize,
    direction: Direction,
}

impl<'a, T> Position<'a, T> {
    /// Creates new Position object
    fn new(grid: &'a Grid<T>, pos: (isize, isize)) -> Position<'a, T> {
        Position {
            grid,
            row: pos.0,
            column: pos.1,
            direction: Direction::Up,
        }
    }

    /// Moves up if that position exists and returns new location, else returns None
    pub fn move_up(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row - 1, self.column) {
            self.row -= 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves down if that position exists and returns new location, else returns None
    pub fn move_down(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row + 1, self.column) {
            self.row += 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves left if that position exists and returns new location, else returns None
    pub fn move_left(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row, self.column - 1) {
            self.column -= 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves right if that position exists in returns new location, else return None
    pub fn move_right(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row, self.column + 1) {
            self.column += 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves forward if possible and returns new position, else returns None
    pub fn move_forward(&mut self) -> Option<(isize, isize)> {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    /// Checks currently faced direction
    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    /// Sets new direction
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }

    /// Turns direction to the right and returns new direction
    pub fn turn_right(&mut self) -> Direction {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }

        self.direction
    }

    /// Turns direction to the left and returns new direction
    pub fn turn_left(&mut self) -> Direction {
        match self.direction {
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Up => self.direction = Direction::Left,
            Direction::Right => self.direction = Direction::Up,
        }

        self.direction
    }

    pub fn get_position(&self) -> (isize, isize) {
        (self.row, self.column)
    }

    pub fn set_position(&mut self, position: (isize, isize)) -> bool {
        if self.grid.contains(position.0, position.1) {
            self.row = position.0;
            self.column = position.1;
            return true;
        } else {
            return false;
        }
    }

    /// Gets reference to item immediately in front of ```self```
    pub fn item_ref_front(&self) -> Option<&T> {
        match self.direction {
            Direction::Up => self.grid.item_ref(self.row - 1, self.column),
            Direction::Down => self.grid.item_ref(self.row + 1, self.column),
            Direction::Left => self.grid.item_ref(self.row, self.column - 1),
            Direction::Right => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    /// Gets reference to item immediately left of ```self```
    pub fn item_ref_left(&self) -> Option<&T> {
        match self.direction {
            Direction::Right => self.grid.item_ref(self.row - 1, self.column),
            Direction::Left => self.grid.item_ref(self.row + 1, self.column),
            Direction::Up => self.grid.item_ref(self.row, self.column - 1),
            Direction::Down => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    /// Gets reference to item immediately right of ```self```
    pub fn item_ref_right(&self) -> Option<&T> {
        match self.direction {
            Direction::Left => self.grid.item_ref(self.row - 1, self.column),
            Direction::Right => self.grid.item_ref(self.row + 1, self.column),
            Direction::Down => self.grid.item_ref(self.row, self.column - 1),
            Direction::Up => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    /// Gets reference to item immediately behind ```self```
    pub fn item_ref_back(&self) -> Option<&T> {
        match self.direction {
            Direction::Down => self.grid.item_ref(self.row - 1, self.column),
            Direction::Up => self.grid.item_ref(self.row + 1, self.column),
            Direction::Right => self.grid.item_ref(self.row, self.column - 1),
            Direction::Left => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    pub fn item_ref(&self) -> &T {
        self.grid.item_ref(self.row, self.column).unwrap()
    }
}

impl<'a, T> PositionMut<'a, T> {
    /// Creates new Position object
    fn new(grid: &'a mut Grid<T>, pos: (isize, isize)) -> Position<'a, T> {
        Position {
            grid,
            row: pos.0,
            column: pos.1,
            direction: Direction::Up,
        }
    }

    /// Moves up if that position exists and returns new location, else returns None
    pub fn move_up(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row - 1, self.column) {
            self.row -= 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves down if that position exists and returns new location, else returns None
    pub fn move_down(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row + 1, self.column) {
            self.row += 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves left if that position exists and returns new location, else returns None
    pub fn move_left(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row, self.column - 1) {
            self.column -= 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves right if that position exists in returns new location, else return None
    pub fn move_right(&mut self) -> Option<(isize, isize)> {
        if self.grid.contains(self.row, self.column + 1) {
            self.column += 1;
            Some((self.row, self.column))
        } else {
            None
        }
    }

    /// Moves forward if possible and returns new position, else returns None
    pub fn move_forward(&mut self) -> Option<(isize, isize)> {
        match self.direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    /// Checks currently faced direction
    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    /// Sets new direction
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction
    }

    /// Turns direction to the right and returns new direction
    pub fn turn_right(&mut self) -> Direction {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }

        self.direction
    }

    /// Turns direction to the left and returns new direction
    pub fn turn_left(&mut self) -> Direction {
        match self.direction {
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Up => self.direction = Direction::Left,
            Direction::Right => self.direction = Direction::Up,
        }

        self.direction
    }

    pub fn get_position(&self) -> (isize, isize) {
        (self.row, self.column)
    }

    pub fn set_position(&mut self, position: (isize, isize)) -> bool {
        if self.grid.contains(position.0, position.1) {
            self.row = position.0;
            self.column = position.1;
            return true;
        } else {
            return false;
        }
    }

    /// Gets reference to item immediately in front of ```self```
    pub fn item_ref_front(&self) -> Option<&T> {
        match self.direction {
            Direction::Up => self.grid.item_ref(self.row - 1, self.column),
            Direction::Down => self.grid.item_ref(self.row + 1, self.column),
            Direction::Left => self.grid.item_ref(self.row, self.column - 1),
            Direction::Right => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    /// Gets reference to item immediately left of ```self```
    pub fn item_ref_left(&self) -> Option<&T> {
        match self.direction {
            Direction::Right => self.grid.item_ref(self.row - 1, self.column),
            Direction::Left => self.grid.item_ref(self.row + 1, self.column),
            Direction::Up => self.grid.item_ref(self.row, self.column - 1),
            Direction::Down => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    /// Gets reference to item immediately right of ```self```
    pub fn item_ref_right(&self) -> Option<&T> {
        match self.direction {
            Direction::Left => self.grid.item_ref(self.row - 1, self.column),
            Direction::Right => self.grid.item_ref(self.row + 1, self.column),
            Direction::Down => self.grid.item_ref(self.row, self.column - 1),
            Direction::Up => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    /// Gets reference to item immediately behind ```self```
    pub fn item_ref_back(&self) -> Option<&T> {
        match self.direction {
            Direction::Down => self.grid.item_ref(self.row - 1, self.column),
            Direction::Up => self.grid.item_ref(self.row + 1, self.column),
            Direction::Right => self.grid.item_ref(self.row, self.column - 1),
            Direction::Left => self.grid.item_ref(self.row, self.column + 1),
        }
    }

    /// Gets mutable reference to item immediately in front of ```self```
    pub fn item_mut_front(&mut self) -> Option<&mut T> {
        match self.direction {
            Direction::Up => self.grid.item_mut(self.row - 1, self.column),
            Direction::Down => self.grid.item_mut(self.row + 1, self.column),
            Direction::Left => self.grid.item_mut(self.row, self.column - 1),
            Direction::Right => self.grid.item_mut(self.row, self.column + 1),
        }
    }

    /// Gets mutable reference to item immediately left of ```self```
    pub fn item_mut_left(&mut self) -> Option<&mut T> {
        match self.direction {
            Direction::Right => self.grid.item_mut(self.row - 1, self.column),
            Direction::Left => self.grid.item_mut(self.row + 1, self.column),
            Direction::Up => self.grid.item_mut(self.row, self.column - 1),
            Direction::Down => self.grid.item_mut(self.row, self.column + 1),
        }
    }

    /// Gets mutable reference to item immediately right of ```self```
    pub fn item_mut_right(&mut self) -> Option<&mut T> {
        match self.direction {
            Direction::Left => self.grid.item_mut(self.row - 1, self.column),
            Direction::Right => self.grid.item_mut(self.row + 1, self.column),
            Direction::Down => self.grid.item_mut(self.row, self.column - 1),
            Direction::Up => self.grid.item_mut(self.row, self.column + 1),
        }
    }

    /// Gets mutable reference to item immediately behind ```self```
    pub fn item_mut_back(&mut self) -> Option<&mut T> {
        match self.direction {
            Direction::Down => self.grid.item_mut(self.row - 1, self.column),
            Direction::Up => self.grid.item_mut(self.row + 1, self.column),
            Direction::Right => self.grid.item_mut(self.row, self.column - 1),
            Direction::Left => self.grid.item_mut(self.row, self.column + 1),
        }
    }

    pub fn item_ref(&self) -> &T {
        self.grid.item_ref(self.row, self.column).unwrap()
    }
}

impl<T> Grid<T> {
    /// Checks if grid contains a set of coordiantes
    /// Uses isize to so that negative values don't need to be checked manually
    pub fn contains(&self, r: isize, c: isize) -> bool {
        (0..self.rows).contains(&(r as usize)) && (0..self.columns).contains(&(c as usize))
    }

    pub fn new() -> Self {
        Self {
            rows: 0,
            columns: 0,
            data: vec![vec![]],
        }
    }

    /// Dimmensions of grid, in the form of (rows, columns)
    pub fn dims(&self) -> (isize, isize) {
        (self.rows as isize, self.columns as isize)
    }

    /// Gets reference to item in grid
    pub fn item_ref(&self, row: isize, column: isize) -> Option<&T> {
        if self.contains(row, column) {
            Some(&self.data[row as usize][column as usize])
        } else {
            None
        }
    }

    /// Gets mutable reference to item in grid
    pub fn item_mut(&mut self, row: isize, column: isize) -> Option<&mut T> {
        if self.contains(row, column) {
            Some(&mut self.data[row as usize][column as usize])
        } else {
            None
        }
    }

    /// Sets value of item in grid, ignores if coordinate is not contained
    pub fn item_set(&mut self, row: isize, column: isize, value: T) {
        if self.contains(row, column) {
            self.data[row as usize][column as usize] = value;
        }
    }

    /// Creation Position object to traverse grid easily
    pub fn traverse(&self, starting_position: (isize, isize)) -> Position<T> {
        Position::new(&self, starting_position)
    }

    pub fn into_vec(self) -> Vec<Vec<T>> {
        self.data
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    /// Assumes all rows have equal length, allows easy creation of Grid from 2 dimmensional Vec
    fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();
        let columns = value.get(0).unwrap_or(&Vec::new()).len();
        let data = value;
        Self {
            rows,
            columns,
            data,
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}
