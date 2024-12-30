use aoc_utils::Grid;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::AddAssign;

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

fn part1(input: &str) -> isize {
    let grid = Grid::from(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>(),
    );

    let grid_size = grid.dims();
    let mut sum = 0;
    for row in 0..grid_size.0 {
        for column in 0..grid_size.1 {
            // print!("{}", grid[(row as usize, column as usize)]);
            if grid[(row as usize, column as usize)] == 0 {
                let mut found: HashSet<(isize, isize)> = HashSet::new();
                count_trails(&grid, 0, (row, column), &mut found);
                sum += found.len();
            }
        }
        // print!("\n");
    }

    sum as isize
}

fn part2(input: &str) -> isize {
    let grid = Grid::from(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>(),
    );

    let grid_size = grid.dims();
    let mut sum = 0;
    for row in 0..grid_size.0 {
        for column in 0..grid_size.1 {
            // print!("{}", grid[(row as usize, column as usize)]);
            if grid[(row as usize, column as usize)] == 0 {
                count_unique_trails(&grid, 0, (row, column), &mut sum);
            }
        }
        // print!("\n");
    }

    sum
}

fn count_trails(
    grid: &Grid<u32>,
    current: u32,
    position: (isize, isize),
    found: &mut HashSet<(isize, isize)>,
) {
    let next_positions = [
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ];
    for pos in next_positions {
        if current == 8 && grid.get(pos.0, pos.1) == Some(9) {
            found.insert(pos);
        } else if grid.get(pos.0, pos.1) == Some(current + 1) {
            count_trails(grid, current + 1, pos, found);
        }
    }
}

fn count_unique_trails(
    grid: &Grid<u32>,
    current: u32,
    position: (isize, isize),
    total: &mut isize
) {
    let next_positions = [
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ];
    for pos in next_positions {
        if current == 8 && grid.get(pos.0, pos.1) == Some(9) {
            total.add_assign(1); 
        } else if grid.get(pos.0, pos.1) == Some(current + 1) {
            count_unique_trails(grid, current + 1, pos, total);
        }
    }
}