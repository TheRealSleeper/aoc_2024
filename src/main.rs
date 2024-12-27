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
    let blocks = input
        .chars()
        .collect::<Vec<char>>()
        .chunks_exact(2)
        .map(|c| (c[0].to_digit(10).unwrap() as u8, c[1].to_digit(10).unwrap() as u8))
        .collect::<Vec<_>>();
    
    let mut uncompressed = vec![]; 
    for (position, block) in blocks.iter().enumerate() {
        for _ in 0..block.0 {
            uncompressed.push(Some(position as u8)); 
        }
        for _ in 0..block.1 {
            uncompressed.push(None); 
        }
    }
    
    let mut i = 0; 
    while i < uncompressed.len() {
        if let None = uncompressed[i] {
            if let Some(end) = uncompressed.pop() {
                if let Some(n) = end {
                    uncompressed.insert(i, Some(n)); 
                }
            }
        }
        
        i += 1;
    }
    
    uncompressed.iter().enumerate().map(|(i, n)| i * n.unwrap_or(0) as usize).sum::<usize>() as isize
}

fn part2(input: &str) -> isize {
    todo!()
}
