use std::fs::read_to_string;

mod aoc_utils;
mod tests;

fn main() {
    let args = aoc_utils::Args::get();
    let sample1 = read_to_string("sample1.txt").expect("Unable to read file");
    let sample2 = read_to_string("sample2.txt").expect("Unable to read file");

    let content = match args.path {
        None => None,
        Some(p) => Some(read_to_string(&p).expect("input: Could not open file")),
    };

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
enum Space {
    Empty,
    Obstacle,
    Visited,
    Guard(Direction),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(input: &str) -> isize {
    let mut guard_position = (0, 0);
    let mut guard_direction = Direction::Up;
    let mut map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Space::Obstacle,
                    '^' => Space::Guard(Direction::Up),
                    'v' => Space::Guard(Direction::Down),
                    '>' => Space::Guard(Direction::Right),
                    '<' => Space::Guard(Direction::Left),
                    _ => Space::Empty,
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<Vec<Space>>>();

    for (i, row) in map.iter().enumerate() {
        for (ii, space) in row.iter().enumerate() {
            if let Space::Guard(dir) = *space {
                guard_position = (i as i32, ii as i32);
                guard_direction = dir;
            }
        }
    }

    loop {
        map[guard_position.0 as usize][guard_position.1 as usize] = Space::Visited;

        match guard_direction {
            Direction::Up => {
                if guard_position.0 <= 0 {
                    break;
                } else if map[guard_position.0 as usize - 1][guard_position.1 as usize]
                    == Space::Obstacle
                {
                    guard_direction = Direction::Right;
                } else {
                    guard_position.0 -= 1;
                }
            }
            Direction::Down => {
                if guard_position.0 >= map.len() as i32 - 1 {
                    break;
                } else if map[guard_position.0 as usize + 1][guard_position.1 as usize]
                    == Space::Obstacle
                {
                    guard_direction = Direction::Left;
                } else {
                    guard_position.0 += 1;
                }
            }
            Direction::Left => {
                if guard_position.1 <= 0 {
                    break;
                } else if map[guard_position.0 as usize][guard_position.1 as usize - 1]
                    == Space::Obstacle
                {
                    guard_direction = Direction::Up;
                } else {
                    guard_position.1 -= 1;
                }
            }
            Direction::Right => {
                if guard_position.1 >= map[guard_position.0 as usize].len() as i32 {
                    break;
                } else if map[guard_position.0 as usize][guard_position.1 as usize + 1]
                    == Space::Obstacle
                {
                    guard_direction = Direction::Down;
                } else {
                    guard_position.1 += 1;
                }
            }
        }
    }

    map.iter().map(|l|l.iter().filter(|s| *s == &Space::Visited).count()).sum::<usize>() as isize
}

fn part2(input: &str) -> isize {
    todo!()
}
