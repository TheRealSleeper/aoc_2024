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

fn part1(input: &str) -> isize {
    input
        .lines()
        .filter_map(|l| {
            let (val, nums) = l.split_once(':').unwrap();
            let val = val.parse::<u64>().unwrap();
            let mut nums = nums
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            nums.reverse();
            let first = nums.pop().unwrap();
            if is_valid(val, first, nums) {
                Some(val)
            } else {
                None
            }
        })
        .sum::<u64>() as isize
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .filter_map(|l| {
            let (val, nums) = l.split_once(':').unwrap();
            let val = val.parse::<u64>().unwrap();
            let mut nums = nums
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            nums.reverse();
            let first = nums.pop().unwrap();
            if is_valid_concat(val, first, nums) {
                Some(val)
            } else {
                None
            }
        })
        .sum::<u64>() as isize
}

fn is_valid(val: u64, current: u64, mut nums: Vec<u64>) -> bool {
    if let Some(n) = nums.pop() {
        is_valid(val, current + n, nums.clone()) || is_valid(val, current * n, nums)
    } else {
        current == val
    }
}

fn is_valid_concat(val: u64, current: u64, mut nums: Vec<u64>) -> bool {
    if let Some(n) = nums.pop() {
        is_valid_concat(val, current + n, nums.clone())
            || is_valid_concat(val, current * n, nums.clone())
            || is_valid_concat(
                val,
                format!("{}{}", current.to_string(), n.to_string())
                    .parse()
                    .unwrap(),
                nums,
            )
    } else {
        current == val
    }
}
