use itertools::{self, Itertools};
use regex::Regex;
use std::fs::read_to_string;
use std::sync::OnceLock;

#[allow(dead_code)]
mod aoc_utils;
mod tests;

static TEST: OnceLock<bool> = OnceLock::new();

fn main() {
    let args = aoc_utils::Args::get();
    let sample1 = read_to_string("sample1.txt").expect("Unable to read file");
    let sample2 = read_to_string("sample2.txt").expect("Unable to read file");
    TEST.get_or_init(|| false);

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

#[derive(Debug, PartialEq, Eq)]
enum Quadrant {
    TL,
    TR,
    BL,
    BR,
    None,
}

static RE_CELL: OnceLock<Regex> = OnceLock::new();

fn part1(_input: &str) -> isize {
    let regex =
        RE_CELL.get_or_init(|| Regex::new(r#"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)"#).unwrap());

    let test = TEST.get_or_init(|| true);

    let grid_size = if *test { (11, 7) } else { (101, 103) };
    let grid_midpoints = (grid_size.0 / 2, grid_size.1 / 2);

    let result = _input
        .lines()
        .map(|l| {
            let robot_data = regex.captures(l).expect("No match found for robot path");
            let final_x = (robot_data[3]
                .parse::<i32>()
                .expect("No horizontal velocity found")
                * 100
                + robot_data[1]
                    .parse::<i32>()
                    .expect("No horizontal starting position found"))
                % grid_size.0;
            let final_y = (robot_data[4]
                .parse::<i32>()
                .expect("No vertical velocity found")
                * 100
                + robot_data[2]
                    .parse::<i32>()
                    .expect("No vertical starting position found"))
                % grid_size.1;

            let final_position = (
                if final_x.is_negative() {
                    final_x + grid_size.0
                } else {
                    final_x
                },
                if final_y.is_negative() {
                    final_y + grid_size.1
                } else {
                    final_y
                },
            );

            if final_position.0 < grid_midpoints.0 && final_position.1 < grid_midpoints.1 {
                Quadrant::TL
            } else if final_position.0 > grid_midpoints.0 && final_position.1 < grid_midpoints.1 {
                Quadrant::TR
            } else if final_position.0 < grid_midpoints.0 && final_position.1 > grid_midpoints.1 {
                Quadrant::BL
            } else if final_position.0 > grid_midpoints.0 && final_position.1 > grid_midpoints.1 {
                Quadrant::BR
            } else {
                Quadrant::None
            }
        })
        .collect::<Vec<_>>();

    (result.iter().filter(|x| *x == &Quadrant::TL).count()
        * result.iter().filter(|x| *x == &Quadrant::TR).count()
        * result.iter().filter(|x| *x == &Quadrant::BL).count()
        * result.iter().filter(|x| *x == &Quadrant::BR).count()) as isize
}

fn part2(_input: &str) -> isize {
    let regex =
        RE_CELL.get_or_init(|| Regex::new(r#"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)"#).unwrap());

    let grid_size = (101, 103);

    let mut robots = _input
        .lines()
        .map(|l| {
            let robot_data = regex.captures(l).unwrap();
            (
                robot_data[1].parse::<i32>().unwrap(),
                robot_data[2].parse::<i32>().unwrap(),
                robot_data[3].parse::<i32>().unwrap(),
                robot_data[4].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut min_size = i32::MAX;
    let mut n_min_size = 0;
    for i in 0..10_000 {
        let mut grid = vec![vec!['.'; grid_size.0]; grid_size.1];
        for robot in robots.iter_mut() {
            grid[robot.1 as usize][robot.0 as usize] = '#';

            robot.0 += robot.2;
            if robot.0 >= grid_size.0 as i32 {
                robot.0 -= grid_size.0 as i32;
            } else if robot.0 < 0 {
                robot.0 += grid_size.0 as i32;
            }

            robot.1 += robot.3;
            if robot.1 >= grid_size.1 as i32 {
                robot.1 -= grid_size.1 as i32;
            } else if robot.1 < 0 {
                robot.1 += grid_size.1 as i32;
            }
        }

        #[allow(unstable_name_collisions)]
        let grid_str = grid
            .into_iter()
            .map(|l| l.iter().collect::<String>())
            .intersperse("\n".into())
            .collect::<String>();

        let avg_size = grid_str
            .as_str()
            .chars()
            .fold((0, '.'), |acc, c| {
                if c != acc.1 {
                    (acc.0 + 1, c)
                } else {
                    (acc.0, c)
                }
            })
            .0;

        if avg_size < min_size {
            min_size = avg_size;
            n_min_size = i;
            println!("{grid_str}\nAfter {i} seconds");
        }
    }
    n_min_size as isize
}
