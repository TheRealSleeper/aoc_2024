use aoc_utils::Direction;
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};
#[cfg(debug_assertions)]
use itertools::Itertools;

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
    WideBoxLeft,
    WideBoxRight,
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

fn check_move(grid: &Vec<Vec<Object>>, dir: Direction, pos: Coord) -> bool {
    if !pos.check_dir(dir) {
        return false;
    }

    let working_pos = pos.pos_neighbor(dir).unwrap();

    match grid[working_pos] {
        Object::Wall => false,
        Object::Space => true,
        Object::Box => check_move(grid, dir, pos.pos_neighbor(dir).unwrap()),
        Object::WideBoxLeft | Object::WideBoxRight => check_wide_box(grid, dir, working_pos),
        Object::Robot => unreachable!(),
    }
}

#[cfg(debug_assertions)]
fn obj_to_char(obj: Object) -> char {
    match obj {
        Object::Box => 'O',
        Object::Robot => '@',
        Object::Space => '.',
        Object::Wall => '#',
        Object::WideBoxLeft => '[',
        Object::WideBoxRight => ']',
    }
}

fn attempt_move(grid: &mut Vec<Vec<Object>>, dir: Direction, pos: Coord) {
    let working_pos = pos.pos_neighbor(dir).unwrap();

    if grid[pos] == Object::Space
        || grid[pos] == Object::WideBoxLeft && dir == Direction::Left
        || grid[pos] == Object::WideBoxRight && dir == Direction::Right
    {
        return;
    }

    match grid[working_pos] {
        Object::Wall => {}
        Object::Space => {
            grid[working_pos] = grid[pos];
            grid[pos] = Object::Space;
        }
        Object::Box => {
            attempt_move(grid, dir, working_pos);
            grid[working_pos] = grid[pos];
            grid[pos] = Object::Space;
        }
        Object::WideBoxLeft | Object::WideBoxRight => {
            let op_side = if grid[working_pos] == Object::WideBoxLeft {
                working_pos.pos_neighbor(Direction::Right).unwrap()
            } else {
                working_pos.pos_neighbor(Direction::Left).unwrap()
            };

            match dir {
                Direction::Down | Direction::Up => {
                    attempt_move(grid, dir, op_side);
                    attempt_move(grid, dir, working_pos);
                    grid[working_pos] = grid[pos];
                    grid[pos] = Object::Space;
                    grid[op_side] = Object::Space;
                }
                Direction::Left | Direction::Right => {
                    let op_side_obj = grid[op_side];
                    grid[op_side] = Object::Box;
                    attempt_move(grid, dir, op_side);
                    grid[op_side] = op_side_obj;
                    grid[op_side.pos_neighbor(dir).unwrap()] = grid[op_side];
                    grid[op_side] = grid[working_pos];
                    grid[working_pos] = grid[pos];
                    grid[pos] = Object::Space;
                }
            }
        }
        Object::Robot => unreachable!(),
    }
}

fn check_wide_box(grid: &Vec<Vec<Object>>, dir: Direction, pos: Coord) -> bool {
    let op_side = match grid[pos] {
        Object::WideBoxLeft => pos.pos_neighbor(Direction::Right).unwrap(),
        Object::WideBoxRight => pos.pos_neighbor(Direction::Left).unwrap(),
        _ => unreachable!(),
    };

    match dir {
        Direction::Left | Direction::Right => match grid[op_side.pos_neighbor(dir).unwrap()] {
            Object::Wall => false,
            Object::Space => true,
            Object::WideBoxLeft | Object::WideBoxRight | Object::Box => {
                check_move(grid, dir, op_side)
            }
            Object::Robot => unreachable!(),
        },
        Direction::Down | Direction::Up => {
            let side1 = match grid[op_side.pos_neighbor(dir).unwrap()] {
                Object::Space => true,
                Object::Wall => false,
                Object::WideBoxLeft | Object::WideBoxRight | Object::Box => {
                    check_move(grid, dir, op_side)
                }
                Object::Robot => unreachable!(),
            };
            let side2 = match grid[pos.pos_neighbor(dir).unwrap()] {
                Object::Space => true,
                Object::Wall => false,
                Object::WideBoxLeft | Object::WideBoxRight | Object::Box => {
                    check_move(grid, dir, pos)
                }
                Object::Robot => unreachable!(),
            };

            side1 && side2
        }
    }
}

#[cfg(debug_assertions)]
fn grid_to_str(grid: &[Vec<Object>]) -> String {
    #[allow(unstable_name_collisions)]
    grid.iter()
        .map(|r| r.iter().map(|o| obj_to_char(*o)).collect::<String>())
        .intersperse("\n".into())
        .collect()
}

fn solve(
    grid: &mut Vec<Vec<Object>>,
    moves: &mut impl Iterator<Item = Direction>,
    pos: &mut Coord,
) -> isize {
    #[cfg(debug_assertions)]
    let box_count = grid
        .iter()
        .map(|r| r.iter().filter(|o| **o == Object::Box).count())
        .sum::<usize>();
    #[cfg(debug_assertions)]
    let box_left_count = grid
        .iter()
        .map(|r| r.iter().filter(|o| **o == Object::WideBoxLeft).count())
        .sum::<usize>();
    #[cfg(debug_assertions)]
    let box_right_count = grid
        .iter()
        .map(|r| r.iter().filter(|o| **o == Object::WideBoxRight).count())
        .sum::<usize>();
    #[cfg(debug_assertions)]
    let wall_count = grid
        .iter()
        .map(|r| r.iter().filter(|o| **o == Object::Wall).count())
        .sum::<usize>();
    #[cfg(debug_assertions)]
    let space_count = grid
        .iter()
        .map(|r| r.iter().filter(|o| **o == Object::Space).count())
        .sum::<usize>();

    for dir in moves {
        #[cfg(debug_assertions)]
        println!(
            "{}\nAt {},{}, Moving {}\n",
            grid_to_str(grid),
            pos.x,
            pos.y,
            dir
        );

        if check_move(grid, dir, *pos) {
            attempt_move(grid, dir, *pos);
            grid[*pos] = Object::Space;
            *pos = pos.pos_neighbor(dir).unwrap();
        }

        #[cfg(debug_assertions)]
        {
            assert_eq!(
                grid.iter()
                    .map(|r| r.iter().filter(|o| **o == Object::Box).count())
                    .sum::<usize>(),
                box_count
            );
            assert_eq!(
                grid.iter()
                    .map(|r| r.iter().filter(|o| **o == Object::WideBoxLeft).count())
                    .sum::<usize>(),
                box_left_count
            );
            assert_eq!(
                grid.iter()
                    .map(|r| r.iter().filter(|o| **o == Object::WideBoxRight).count())
                    .sum::<usize>(),
                box_right_count
            );
            assert_eq!(
                grid.iter()
                    .map(|r| r.iter().filter(|o| **o == Object::Wall).count())
                    .sum::<usize>(),
                wall_count
            );
            assert_eq!(
                grid.iter()
                    .map(|r| r.iter().filter(|o| **o == Object::Space).count())
                    .sum::<usize>(),
                space_count
            );
            assert_eq!(
                grid.iter()
                    .map(|r| r.iter().filter(|o| **o == Object::Robot).count())
                    .sum::<usize>(),
                1
            );
            // std::thread::sleep(std::time::Duration::from_millis(60));
        }
    }

    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(ii, obj)| {
                    if *obj == Object::Box || *obj == Object::WideBoxLeft {
                        i * 100 + ii
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>() as isize
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

    let mut moves = moves_str.lines().flat_map(|l| {
        l.chars().map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
    });

    solve(&mut grid, &mut moves, &mut position)
}

fn part2(_input: &str) -> isize {
    let (map_str, moves_str) = _input
        .split_once(if cfg!(windows) { "\r\n\r\n" } else { "\n\n" })
        .expect("Invalid Input");
    let width = map_str.lines().next().unwrap().chars().count();
    let height = map_str.lines().count();
    let mut position = Coord {
        x: 0,
        y: 0,
        max_x: width * 2 - 1,
        max_y: height * 2 - 1,
    };

    let mut grid = map_str
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .flat_map(|(ii, c)| match c {
                    '.' => [Object::Space; 2],
                    'O' => [Object::WideBoxLeft, Object::WideBoxRight],
                    '@' => {
                        position.x = ii * 2;
                        position.y = i;
                        [Object::Robot, Object::Space]
                    }
                    '#' => [Object::Wall; 2],
                    _ => panic!("Invalid object"),
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut moves = moves_str.lines().flat_map(|l| {
        l.chars().map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction"),
        })
    });

    solve(&mut grid, &mut moves, &mut position)
}
