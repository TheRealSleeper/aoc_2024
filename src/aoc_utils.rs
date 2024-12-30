use std::env::args;
use std::ops::Index;
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

#[derive(Clone, Debug)]
pub struct Grid<T> {
    rows: usize,
    columns: usize,
    data: Vec<Vec<T>>,
}

#[allow(dead_code)]
impl<T> Grid<T> {
    /// Checks if grid contains a set of coordiantes
    /// Uses isize to so that negative values don't need to be checked manually
    pub fn contains(&self, r: isize, c: isize) -> bool {
        (0..self.rows as isize).contains(&r) && (0..self.columns as isize).contains(&c)
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
        if self.contains(row as isize, column as isize) {
            Some(&self.data[row as usize][column as usize])
        } else {
            None
        }
    }

    /// Gets mutable reference to item in grid
    pub fn item_mut(&mut self, row: isize, column: isize) -> Option<&mut T> {
        if self.contains(row as isize, column as isize) {
            Some(&mut self.data[row as usize][column as usize])
        } else {
            None
        }
    }

    /// Sets value of item in grid, ignores if coordinate is not contained
    pub fn item_set(&mut self, row: isize, column: isize, value: T) {
        if self.contains(row as isize, column as isize) {
            self.data[row as usize][column as usize] = value;
        }
    }
}

#[allow(dead_code)]
impl<T: Copy> Grid<T> {
    pub fn get(&self, row: isize, column: isize) -> Option<T> {
        if self.contains(row as isize, column as isize) {
            Some(self.data[row as usize][column as usize])
        } else {
            None
        }
    }
}

#[allow(dead_code)]
impl<T: ToOwned> Grid<T> {
    pub fn get_owned(&self, row: isize, column: isize) -> Option<<T as ToOwned>::Owned> {
        if self.contains(row as isize, column as isize) {
            Some(self.data[row as usize][column as usize].to_owned())
        } else {
            None
        }
    }
}

use std::ops::IndexMut;
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

#[allow(dead_code)]
impl<T> From<Vec<Vec<T>>> for Grid<T> {
    /// Assumes all rows have equal length
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
