use std::fs::read_to_string;
use std::ops::AddAssign;

use aoc_utils::Grid;

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
    let mut plots = Grid::from(
        _input
            .lines()
            .map(|l| l.chars().map(|c| (c, false)).collect())
            .collect::<Vec<Vec<(char, bool)>>>(),
    );
    let (rows, columns) = plots.dims();
    let mut regions: Vec<(Vec<((isize, isize), usize)>, char)> = vec![];
    for row in 0..rows {
        for column in 0..columns {
            if !plots[(row as usize, column as usize)].1 {
                regions.push((Vec::new(), plots[(row as usize, column as usize)].0));
                let plot_type = plots[(row as usize, column as usize)].0;
                get_region(
                    &mut plots,
                    plot_type,
                    (row, column),
                    &mut regions.last_mut().unwrap().0,
                );
            }
        }
    }

    regions.into_iter().fold(0, |sum, region| {
        sum + region.0.len()
            * region
                .0
                .iter()
                .fold(0, |perimeter, plot| perimeter + 4 - plot.1)
    }) as isize
}

fn part2(_input: &str) -> isize {
    let mut plots = Grid::from(
        _input
            .lines()
            .map(|l| l.chars().map(|c| (c, false)).collect())
            .collect::<Vec<Vec<(char, bool)>>>(),
    );
    let (rows, columns) = plots.dims();
    let mut regions: Vec<(Vec<((isize, isize), usize)>, char)> = vec![];
    for row in 0..rows {
        for column in 0..columns {
            if !plots[(row as usize, column as usize)].1 {
                regions.push((Vec::new(), plots[(row as usize, column as usize)].0));
                let plot_type = plots[(row as usize, column as usize)].0;
                get_region(
                    &mut plots,
                    plot_type,
                    (row, column),
                    &mut regions.last_mut().unwrap().0,
                );
            }
        }
    }
    
    let mut sides = vec![]; 
    for region in regions.iter() {
        let mut current_sides: usize = 0; 
        let starting_position = region.0[0].0; 
        let mut traverser = plots.traverse(starting_position); 
        traverser.set_direction(aoc_utils::Direction::Right); 
        let plot_type = traverser.item_ref().0; 
        
        while (traverser.get_position(), traverser.get_direction()) != ((starting_position, aoc_utils::Direction::Right)) {
            if let Some(item) = traverser.item_ref_left() {
                if item.0 == plot_type {
                    traverser.turn_left(); 
                    current_sides += 1; 
                }
            } else if let Some(item) = traverser.item_ref_right() {
                if item.0 == plot_type {
                    traverser.turn_right(); 
                    current_sides += 1; 
                }
            } else if let Some(item) = traverser.item_ref_front() {
                if item.0 == plot_type {
                    traverser.move_forward(); 
                }
            }
        }
        
        sides.push(current_sides); 
    }
    
    sides.into_iter().zip(regions.into_iter()).map(|(sides, (positions, _))| sides * positions.len()).sum::<usize>() as isize
}

fn get_region(
    plots: &mut Grid<(char, bool)>,
    plot_type: char,
    position: (isize, isize),
    region_plots: &mut Vec<((isize, isize), usize)>,
) {
    if plots.item_ref(position.0, position.1).unwrap().0 == plot_type && !plots.item_ref(position.0, position.1).unwrap().1 {
        region_plots.push((position, 0));
        plots.item_mut(position.0, position.1).unwrap().1 = true;

        if let Some(position_new) = plots.traverse(position).move_up() {
            get_region(plots, plot_type, position_new, region_plots);
            if plots.item_ref(position_new.0, position_new.1).unwrap().0 == plot_type {
                region_plots.last_mut().unwrap().1.add_assign(1);
            }
        }

        if let Some(position_new) = plots.traverse(position).move_down() {
            get_region(plots, plot_type, position_new, region_plots);
            if plots.item_ref(position_new.0, position_new.1).unwrap().0 == plot_type {
                region_plots.last_mut().unwrap().1.add_assign(1);
            }        
        }

        if let Some(position_new) = plots.traverse(position).move_left() {
            get_region(plots, plot_type, position_new, region_plots);
            if plots.item_ref(position_new.0, position_new.1).unwrap().0 == plot_type {
                region_plots.last_mut().unwrap().1.add_assign(1);
            }
        }

        if let Some(position_new) = plots.traverse(position).move_right() {
            get_region(plots, plot_type, position_new, region_plots);
            if plots.item_ref(position_new.0, position_new.1).unwrap().0 == plot_type {
                region_plots.last_mut().unwrap().1.add_assign(1);
            }
        }
    }
}
