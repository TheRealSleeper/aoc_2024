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

const STEP_MAX: u8 = 3;
const STEP_MIN: u8 = 1;

#[derive(Debug, PartialEq, Clone, Copy)]
enum GroupType {
    Undecided,
    Up,
    Down,
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|w| w.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .filter(check_group)
        .count() as isize
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|w| w.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .filter(|g| {
            if !check_group(g) {
                for i in 0..g.len() {
                    let mut tmp = g.clone(); 
                    tmp.remove(i); 
                    if check_group(&tmp) {
                        return true;
                    }
                }

                return false;
            } else {
                true
            }
        })
        .count() as isize
}

fn check_group(inp: &Vec<u8>) -> bool {
    let mut state = GroupType::Undecided;
    for w in inp.windows(2) {
        match state {
            GroupType::Undecided => {
                if w[0] < w[1] && w[0].abs_diff(w[1]) >= STEP_MIN && w[0].abs_diff(w[1]) <= STEP_MAX {
                    state = GroupType::Up;
                } else if w[0] > w[1] && w[0].abs_diff(w[1]) >= STEP_MIN && w[0].abs_diff(w[1]) <= STEP_MAX {
                    state = GroupType::Down;
                } else {
                    return false;
                }
            }

            GroupType::Up => {
                if w[0] > w[1] || w[0].abs_diff(w[1]) < STEP_MIN || w[0].abs_diff(w[1]) > STEP_MAX {
                    return false;
                }
            }

            GroupType::Down => {
                if w[0] < w[1] || w[0].abs_diff(w[1]) < STEP_MIN || w[0].abs_diff(w[1]) > STEP_MAX {
                    return false;
                }
            }
        }
    }

    return true;
}
