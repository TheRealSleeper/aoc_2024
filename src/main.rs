use std::fs::read_to_string;

mod aoc_utils;
mod tests;

fn main() {
    let args = aoc_utils::Args::get();
    let sample1 = read_to_string("sample1.txt").expect("Unable to read file");
    let sample2 = read_to_string("sample2.txt").expect("Unable to read file");

    let content = match args.path {
        None => None,
        Some(p) => Some(
            read_to_string(&p)
                .expect("input: Could not open file")
                .trim()
                .to_owned(),
        ),
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
    let (pairs, data) = _input.split_once("\r\n\r\n").unwrap();
    let pairs = pairs
        .lines()
        .map(|l| {
            let group = l.split_once('|').unwrap();
            return (
                group.0.parse::<u8>().unwrap(),
                group.1.parse::<u8>().unwrap(),
            );
        })
        .collect::<Vec<(u8, u8)>>();

    let lines = data
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    lines
        .iter()
        .filter(|l| {
            let mut found: Vec<u8> = vec![];
            for n in *l {
                for f in &found {
                    for (l, h) in &pairs {
                        if n == l && f == h {
                            return false;
                        }
                    }
                }
                found.push(*n);
            }
            return true;
        })
        .map(|l| l[l.len() / 2] as isize)
        .sum::<isize>()
}

fn part2(_input: &str) -> isize {
    let (pairs, data) = _input.split_once("\r\n\r\n").unwrap();
    let pairs = pairs
        .lines()
        .map(|l| {
            let group = l.split_once('|').unwrap();
            return (
                group.0.parse::<u8>().unwrap(),
                group.1.parse::<u8>().unwrap(),
            );
        })
        .collect::<Vec<(u8, u8)>>();

    let lines = data
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut lines = lines
        .iter()
        .filter(|l| {
            let mut found: Vec<u8> = vec![];
            for n in *l {
                for f in &found {
                    for (l, h) in &pairs {
                        if n == l && f == h {
                            return true;
                        }
                    }
                }
                found.push(*n);
            }
            return false;
        })
        .map(|l| l.clone())
        .collect::<Vec<Vec<u8>>>();

    for line in &mut lines {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..line.len() {
                for ii in i + 1..line.len() {
                    for (l, h) in &pairs {
                        if line[i] == *h && line[ii] == *l {
                            line.swap(i, i + 1);
                            swapped = true;
                        }
                    }
                }
            }
        }
    }

    lines.iter().map(|l| l[l.len() / 2] as isize).sum::<isize>()
}
