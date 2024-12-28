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
        .chunks(2)
        .map(|c| {
            (
                c[0].to_digit(10).unwrap(),
                c.get(1).unwrap_or(&'0').to_digit(10).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut uncompressed = vec![];
    for (position, block) in blocks.iter().enumerate() {
        for _ in 0..block.0 {
            uncompressed.push(Some(position));
        }
        for _ in 0..block.1 {
            uncompressed.push(None);
        }
    }

    let mut i = 0;
    while i < uncompressed.len() {
        if uncompressed[i] == None {
            let mut end_digit = uncompressed.pop().unwrap();
            while end_digit == None && i < uncompressed.len() {
                end_digit = uncompressed.pop().unwrap();
            }
            uncompressed[i] = end_digit;
        }
        i += 1;
    }

    uncompressed
        .iter()
        .enumerate()
        .map(|(i, n)| i * n.unwrap_or(0) as usize)
        .sum::<usize>() as isize
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DiskSpace {
    Free(usize),
    Block((usize, usize)),
}

fn part2(input: &str) -> isize {
    let blocks = input
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| {
            (
                c[0].to_digit(10).unwrap(),
                c.get(1).unwrap_or(&'0').to_digit(10).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut uncompressed = vec![];
    for (i, block) in blocks.iter().enumerate() {
        uncompressed.push(DiskSpace::Block((i, block.0 as usize)));
        uncompressed.push(DiskSpace::Free(block.1 as usize));
    }
    
    'outer: for i in (0..uncompressed.len()).rev() {
        let block = if let DiskSpace::Block((id, size)) = uncompressed[i] {
            (id, size)
        } else {
            continue 'outer;
        };

        'inner: for ii in 0..i {
            if let DiskSpace::Free(free_space) = uncompressed[ii] {
                if free_space >= block.1 {
                    let file = uncompressed.remove(i);
                    uncompressed[ii] = DiskSpace::Free(free_space - block.1);
                    uncompressed.insert(ii, file);
                    if let DiskSpace::Free(n) = uncompressed[i] {
                        uncompressed[i] = DiskSpace::Free(n + block.1);
                    } else {
                        uncompressed.insert(i, DiskSpace::Free(block.1)); 
                    }
                    break 'inner;
                }
            }
        }
    }

    let mut compressed = vec![];
    for block in uncompressed {
        match block {
            DiskSpace::Free(n) => {
                for _ in 0..n {
                    compressed.push(None);
                }
            }
            DiskSpace::Block((id, size)) => {
                for _ in 0..size {
                    compressed.push(Some(id));
                }
            }
        }
    }

    compressed
        .iter()
        .enumerate()
        .map(|(i, block)| block.unwrap_or(0) * i)
        .sum::<usize>() as isize
}
