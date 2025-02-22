use regex::RegexBuilder;
use std::fs::read_to_string;

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
    let machines_str = _input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(4)
        .map(|chunk| format!("{}; {}; {}", chunk[0], chunk[1], chunk[2]))
        .collect::<Vec<String>>();

    let button_pattern = RegexBuilder::new(r#"Button (?<letter>A|B): X\+(?<x>\d+), Y\+(?<y>\d+)"#)
        .case_insensitive(true)
        .multi_line(true)
        .build()
        .unwrap();

    let prize_pattern = RegexBuilder::new(r#"Prize: X=(?<x>\d+), Y=(?<y>\d+)"#)
        .case_insensitive(true)
        .multi_line(true)
        .build()
        .unwrap();

    machines_str
        .iter()
        .map(|machine| {
            let mut a = (0.0, 0.0);
            let mut b = (0.0, 0.0);

            for caps in button_pattern.captures_iter(&machine) {
                if caps.name("letter").unwrap().as_str() == "A" {
                    a = (
                        caps.name("x").unwrap().as_str().parse().unwrap(),
                        caps.name("y").unwrap().as_str().parse().unwrap(),
                    );
                } else {
                    b = (
                        caps.name("x").unwrap().as_str().parse().unwrap(),
                        caps.name("y").unwrap().as_str().parse().unwrap(),
                    );
                }
            }

            let prize: (f64, f64) = {
                let caps = prize_pattern.captures(&machine).unwrap();
                (
                    caps.name("x").unwrap().as_str().parse::<isize>().unwrap() as f64,
                    caps.name("y").unwrap().as_str().parse::<isize>().unwrap() as f64,
                )
            };

            // Use Cramer's rule to solve
            let d = a.0 * b.1 - a.1 * b.0;
            let da = prize.0 * b.1 - prize.1 * b.0;
            let db = a.0 * prize.1 - a.1 * prize.0;

            if std::env::var("AOC_VERBOSE")
                .unwrap_or("".to_string())
                .as_str()
                != ""
            {
                println!("{}", machine);
                println!(
                    "A: ({}, {}), B: ({}, {}), Prize: ({}, {})",
                    a.0, a.1, b.0, b.1, prize.0, prize.1
                );
            }

            if (da / d) == (da / d).floor() && (db / d) == (db / d).floor() {
                ((da / d) * 3.0 + db / d) as isize
            } else {
                0
            }
        })
        .sum()
}

fn part2(_input: &str) -> isize {
    let machines_str = _input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(4)
        .map(|chunk| format!("{}; {}; {}", chunk[0], chunk[1], chunk[2]))
        .collect::<Vec<String>>();

    let button_pattern = RegexBuilder::new(r#"Button (?<letter>A|B): X\+(?<x>\d+), Y\+(?<y>\d+)"#)
        .case_insensitive(true)
        .multi_line(true)
        .build()
        .unwrap();

    let prize_pattern = RegexBuilder::new(r#"Prize: X=(?<x>\d+), Y=(?<y>\d+)"#)
        .case_insensitive(true)
        .multi_line(true)
        .build()
        .unwrap();

    machines_str
        .iter()
        .map(|machine| {
            let mut a = (0.0, 0.0);
            let mut b = (0.0, 0.0);

            for caps in button_pattern.captures_iter(&machine) {
                if caps.name("letter").unwrap().as_str() == "A" {
                    a = (
                        caps.name("x").unwrap().as_str().parse().unwrap(),
                        caps.name("y").unwrap().as_str().parse().unwrap(),
                    );
                } else {
                    b = (
                        caps.name("x").unwrap().as_str().parse().unwrap(),
                        caps.name("y").unwrap().as_str().parse().unwrap(),
                    );
                }
            }

            let prize: (f64, f64) = {
                let caps = prize_pattern.captures(&machine).unwrap();
                (
                    caps.name("x").unwrap().as_str().parse::<isize>().unwrap() as f64
                        + 10000000000000.0,
                    caps.name("y").unwrap().as_str().parse::<isize>().unwrap() as f64
                        + 10000000000000.0,
                )
            };

            // Use cramer's rule to solve
            let d = a.0 * b.1 - a.1 * b.0;
            let da = prize.0 * b.1 - prize.1 * b.0;
            let db = a.0 * prize.1 - a.1 * prize.0;

            if std::env::var("AOC_VERBOSE")
                .unwrap_or("".to_string())
                .as_str()
                != ""
            {
                println!("{}", machine);
                println!(
                    "A: ({}, {}), B: ({}, {}), Prize: ({}, {})",
                    a.0, a.1, b.0, b.1, prize.0, prize.1
                );
            }

            if (da / d) == (da / d).floor() && (db / d) == (db / d).floor() {
                ((da / d) * 3.0 + db / d) as isize
            } else {
                0
            }
        })
        .sum()
}
