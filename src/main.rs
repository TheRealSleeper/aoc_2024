use aoc_utils::Direction;
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

#[allow(dead_code)]
mod aoc_utils;
mod tests;

fn main() {
    let args = aoc_utils::Args::get();
    let sample1 = read_to_string("sample1a.txt").expect("Unable to read file");
    let sample2 = read_to_string("sample2.txt").expect("Unable to read file");

    let content = args
        .path
        .map(|p| read_to_string(&p).expect("input: Could not open file"));

    if args.part1 {
        println!(
            "{}",
            if args.sample {
                part1(&sample1)
            } else {
                part1(content.as_deref().expect("No input file was opened"))
            }
        )
    }

    if args.part2 {
        println!(
            "{}",
            if args.sample {
                part2(&sample2)
            } else {
                part2(content.as_deref().expect("No input file was opened"))
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Object {
    Wall,
    Box,
    Robot,
    Space,
}

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
}

impl Coord {
    fn check_dir(&self, dir: Direction) -> bool {
        match dir {
            Direction::Left => self.x > 0,
            Direction::Right => self.x < self.max_x,
            Direction::Up => self.y > 0,
            Direction::Down => self.y < self.max_y,
        }
    }

    fn pos_neighbor(&self, dir: Direction) -> Option<Self> {
        if !self.check_dir(dir) {
            None
        } else {
            Some(match dir {
                Direction::Left => Coord {
                    x: self.x - 1,
                    y: self.y,
                    max_x: self.max_x,
                    max_y: self.max_y,
                },
                Direction::Right => Coord {
                    x: self.x + 1,
                    y: self.y,
                    max_x: self.max_x,
                    max_y: self.max_y,
                },
                Direction::Up => Coord {
                    x: self.x,
                    y: self.y - 1,
                    max_x: self.max_x,
                    max_y: self.max_y,
                },
                Direction::Down => Coord {
                    x: self.x,
                    y: self.y + 1,
                    max_x: self.max_x,
                    max_y: self.max_y,
                },
            })
        }
    }
}

impl<T> Index<Coord> for Vec<Vec<T>> {
    type Output = T;
    fn index(&self, index: Coord) -> &Self::Output {
        self.index(index.y).index(index.x)
    }
}

impl<T> IndexMut<Coord> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.index_mut(index.y).index_mut(index.x)
    }
}

fn attempt_move(grid: &mut Vec<Vec<Object>>, dir: Direction, pos: Coord) -> bool {
    if !pos.check_dir(dir) {
        return false;
    }

    match grid[pos.pos_neighbor(dir).unwrap()] {
        Object::Wall => false,
        Object::Space => {
            grid[pos.pos_neighbor(dir).unwrap()] = grid[pos];
            grid[pos] = Object::Space;
            true
        }
        Object::Box => {
            if attempt_move(grid, dir, pos.pos_neighbor(dir).unwrap()) {
                grid[pos.pos_neighbor(dir).unwrap()] = grid[pos];
                grid[pos] = Object::Space;
                true
            } else {
                false
            }
        }
        Object::Robot => unreachable!(),
    }
}

fn part1(_input: &str) -> isize {
    let (map_str, moves_str) = _input
        .split_once(if cfg!(windows) { "\r\n\r\n" } else { "\n\n" })
        .expect("Invalid Input");
    let width = map_str.lines().next().unwrap().chars().count();
    let height = map_str.lines().count();
    let mut position = Coord {
        x: 0,
        y: 0,
        max_x: width - 1,
        max_y: height - 1,
    };

    let mut grid = map_str
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(ii, c)| match c {
                    '.' => Object::Space,
                    'O' => Object::Box,
                    '@' => {
                        position.x = ii;
                        position.y = i;
                        Object::Robot
                    }
                    '#' => Object::Wall,
                    _ => panic!("Invalid object"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let moves = moves_str.lines().flat_map(|l| {
        l.chars().map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
    });

    for dir in moves {
        if attempt_move(&mut grid, dir, position) {
            grid[position] = Object::Space;
            position = position.pos_neighbor(dir).unwrap();
        }
    }

    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(
                    |(ii, obj)| {
                        if *obj == Object::Box {
                            i * 100 + ii
                        } else {
                            0
                        }
                    },
                )
                .sum::<usize>()
        })
        .sum::<usize>() as isize
}

fn part2(_input: &str) -> isize {
    todo!()
}
