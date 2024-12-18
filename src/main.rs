use std::{fs::read_to_string, ops::AddAssign};

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

fn search_all_dir(map: &Vec<Vec<char>>, target_row: usize, target_col: usize, accum: &mut isize) {
    for r in target_row.saturating_sub(1)..(target_row + 1).clamp(0, map.len() - 1) {
        for c in target_col.saturating_sub(1)..(target_col + 1).clamp(0, map[r].len() - 1) {
            search_directional(
                map,
                r,
                c,
                r as isize - target_row as isize,
                c as isize - target_col as isize,
                'M',
                accum,
            );
        }
    }
}

fn search_directional(
    map: &Vec<Vec<char>>,
    target_row: usize,
    target_col: usize,
    dir_row: isize,
    dir_col: isize,
    target_char: char,
    accum: &mut isize,
) {
    let current_char = map[target_row][target_col]; 
        // println!("Current character is {} at row {}, column {}", 
        //     current_char, 
        //     target_row + 1, 
        //     target_col + 1); 
        if target_char == current_char {
            println!(
                "\'{}\' at row {}, column {}",
                target_char,
                target_row + 1,
                target_col + 1
            );
            match target_char {
                'M' => {
                    if target_row as isize + (dir_row as isize) >= 0
                        && target_row as isize + (dir_row as isize) < map.len() as isize
                        && target_col as isize + (dir_col as isize) >= 0
                        && target_col as isize + (dir_col as isize) < map[target_row].len() as isize
                    {
                        search_directional(
                            map,
                            target_row.saturating_add_signed(dir_row),
                            target_col.saturating_add_signed(dir_col), 
                            dir_row,
                            dir_col,
                            'A',
                            accum,
                        );
                    }
                }
    
                'A' => {
                    if target_row as isize + (dir_row as isize) >= 0
                        && target_row as isize + (dir_row as isize) < map.len() as isize
                        && target_col as isize + (dir_col as isize) >= 0
                        && target_col as isize + (dir_col as isize) < map[target_row].len() as isize
                    {
                        search_directional(
                            map, 
                            target_row.saturating_add_signed(dir_row),
                            target_col.saturating_add_signed(dir_col), 
                            dir_row,
                            dir_col,
                            'S',
                            accum,
                        );
                    }
                }
    
                'S' => {
                    accum.add_assign(1);
                }
    
                _ => {}
            }
        }
}

fn part1(input: &str) -> isize {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count: isize = 0;

    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == 'X' {
                println!("\'X\' at row {}, column {}", row + 1, column + 1);
                search_all_dir(&map, row, column, &mut count);
            }
        }
    }

    return count;
}

fn part2(input: &str) -> isize {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count: isize = 0;

    for row in 1..map.len() - 1 {
        for column in 1..map[row].len() - 1 {
            if map[row][column] == 'A' {
                let check = [
                    map[row - 1][column - 1],
                    map[row - 1][column + 1],
                    map[row + 1][column - 1],
                    map[row + 1][column + 1],
                ];
                match check {
                    ['M', 'M', 'S', 'S'] => count += 1,
                    ['S', 'M', 'S', 'M'] => count += 1,
                    ['S', 'S', 'M', 'M'] => count += 1,
                    ['M', 'S', 'M', 'S'] => count += 1,
                    _ => {}
                }
            }
        }
    }
    return count;
}
