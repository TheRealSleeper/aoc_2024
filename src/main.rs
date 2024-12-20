use regex::RegexBuilder;
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
    let re = RegexBuilder::new(r"mul\((?<n1>\d+),(?<n2>\d+)\)")
        .build()
        .unwrap();
    re.captures_iter(input).fold(0, |a, c| {
        a + c["n1"].parse::<isize>().unwrap() * c["n2"].parse::<isize>().unwrap()
    })
}

fn part2(input: &str) -> isize {
    let re = RegexBuilder::new(r"(?:mul\((?<n1>\d+),(?<n2>\d+)\))|(?<dont>don't)|(?<do>do)")
        .build()
        .unwrap();
    let mut mul = true;

    re.captures_iter(input).fold(0, |a, c| {
        if let Some(_) = c.name("do") {
            mul = true;
            a
        } else if let Some(_) = c.name("dont") {
            mul = false;
            a
        } else if mul {
            a + c["n1"].parse::<isize>().unwrap() * c["n2"].parse::<isize>().unwrap()
        } else {
            a
        }
    })
}