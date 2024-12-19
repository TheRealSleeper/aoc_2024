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
    let raw_nums = input
        .lines()
        .map(|l| {
            let p = l.split_once("   ").unwrap();
            (p.0.parse::<u32>().unwrap(), p.1.parse::<u32>().unwrap())
        })
        .collect::<Vec<(u32, u32)>>();

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    for (l, r) in (raw_nums) {
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|p| p.0.abs_diff(*p.1))
        .sum::<u32>() as isize
}

fn part2(input: &str) -> isize {
    todo!()
}
