use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::AddAssign;

#[allow(dead_code)]
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

fn part1(_input: &str) -> isize {
    let mut stones: HashMap<usize, usize> = _input
        .split_whitespace()
        .map(|stone| (stone.parse::<usize>().unwrap(), 1))
        .collect();
    for _ in 0..25 {
        let mut stones_new: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in stones {
            if stone == 0 {
                stones_new.entry(1).or_insert(0).add_assign(count);
            } else if stone.to_string().chars().count() % 2 == 0 {
                let stone_str = stone.to_string();
                let stone1 = stone_str[0..stone_str.len() / 2].parse::<usize>().unwrap();
                let stone2 = stone_str[stone_str.len() / 2..].parse::<usize>().unwrap();
                stones_new.entry(stone1).or_insert(0).add_assign(count);
                stones_new.entry(stone2).or_insert(0).add_assign(count);
            } else {
                stones_new
                    .entry(stone * 2024)
                    .or_insert(0)
                    .add_assign(count);
            }
        }

        stones = stones_new;
    }

    stones
        .into_iter()
        .fold(0, |total, (_, count)| total + count) as isize
}

fn part2(_input: &str) -> isize {
    let mut stones: HashMap<usize, usize> = _input
        .split_whitespace()
        .map(|stone| (stone.parse::<usize>().unwrap(), 1))
        .collect();
    for _ in 0..75 {
        let mut stones_new: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in stones {
            if stone == 0 {
                stones_new.entry(1).or_insert(0).add_assign(count);
            } else if stone.to_string().chars().count() % 2 == 0 {
                let stone_str = stone.to_string();
                let stone1 = stone_str[0..stone_str.len() / 2].parse::<usize>().unwrap();
                let stone2 = stone_str[stone_str.len() / 2..].parse::<usize>().unwrap();
                stones_new.entry(stone1).or_insert(0).add_assign(count);
                stones_new.entry(stone2).or_insert(0).add_assign(count);
            } else {
                stones_new
                    .entry(stone * 2024)
                    .or_insert(0)
                    .add_assign(count);
            }
        }

        stones = stones_new;
    }

    stones
        .into_iter()
        .fold(0, |total, (_, count)| total + count) as isize
}
