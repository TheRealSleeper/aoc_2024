use std::collections::HashMap;
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

#[derive(Clone, Debug)]
struct Grid<T> {
    rows: usize,
    columns: usize,
    data: Vec<Vec<T>>,
}

#[allow(dead_code)]
impl<T> Grid<T> {
    fn contains(&self, r: isize, c: isize) -> bool {
        (0..self.rows as isize).contains(&r) && (0..self.columns as isize).contains(&c)
    }

    fn new() -> Self {
        Self {
            rows: 0,
            columns: 0,
            data: vec![vec![]],
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    /// Assumes all rows have equal length
    fn from(value: Vec<Vec<T>>) -> Self {
        let rows = value.len();
        let columns = value.get(0).unwrap_or(&Vec::new()).len();
        let data = value;
        Self {
            rows,
            columns,
            data,
        }
    }
}

fn part1(input: &str) -> isize {
    let input_grid: Grid<char> = Grid::from(
        input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );

    let valid_chars = ('0'..='9')
        .chain('a'..='z')
        .chain('A'..='Z')
        .collect::<Vec<char>>();

    let mut antennae: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in input_grid.data.iter().enumerate() {
        for (ii, antenna) in row.iter().enumerate() {
            if valid_chars.contains(antenna) {
                antennae.entry(*antenna).or_insert(Vec::new()).push((i, ii));
            }
        }
    }

    let mut antinodes = Grid::from(vec![vec![false; input_grid.columns]; input_grid.rows]);
    for (_, locations) in antennae {
        for i in 0..locations.len() - 1 {
            for ii in i + 1..locations.len() {
                let direction = (
                    locations[ii].0 as isize - locations[i].0 as isize,
                    locations[ii].1 as isize - locations[i].1 as isize,
                );

                let location1 = (
                    locations[i].0 as isize - direction.0,
                    locations[i].1 as isize - direction.1,
                );
                if antinodes.contains(location1.0, location1.1) {
                    antinodes.data[location1.0 as usize][location1.1 as usize] = true;
                }

                let location2 = (
                    locations[ii].0 as isize + direction.0,
                    locations[ii].1 as isize + direction.1,
                );
                if antinodes.contains(location2.0, location2.1) {
                    antinodes.data[location2.0 as usize][location2.1 as usize] = true;
                }
            }
        }
    }

    antinodes
        .data
        .iter()
        .map(|rows| rows.iter().filter(|space| **space).count())
        .sum::<usize>() as isize
}

fn part2(input: &str) -> isize {
    let input_grid: Grid<char> = Grid::from(
        input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    );

    let valid_chars = ('0'..='9')
        .chain('a'..='z')
        .chain('A'..='Z')
        .collect::<Vec<char>>();

    let mut antennae: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in input_grid.data.iter().enumerate() {
        for (ii, antenna) in row.iter().enumerate() {
            if valid_chars.contains(antenna) {
                antennae.entry(*antenna).or_insert(Vec::new()).push((i, ii));
            }
        }
    }

    let mut antinodes = Grid::from(vec![vec![false; input_grid.columns]; input_grid.rows]);
    for (_, locations) in antennae {
        for i in 0..locations.len() - 1 {
            for ii in i + 1..locations.len() {
                let direction = (
                    locations[ii].0 as isize - locations[i].0 as isize,
                    locations[ii].1 as isize - locations[i].1 as isize,
                );
                
                let mut starting_location = (locations[i].0 as isize, locations[i].1 as isize); 
                while antinodes.contains(starting_location.0, starting_location.1) {
                    starting_location.0 -= direction.0; 
                    starting_location.1 -= direction.1; 
                }

                let mut location = (starting_location.0 + direction.0, starting_location.1 + direction.1); 
                
                while antinodes.contains(location.0, location.1) {
                    antinodes.data[location.0 as usize][location.1 as usize] = true; 
                    location.0 += direction.0; 
                    location.1 += direction.1; 
                }
            }
        }
    }

    antinodes
        .data
        .iter()
        .map(|rows| rows.iter().filter(|space| **space).count())
        .sum::<usize>() as isize}
