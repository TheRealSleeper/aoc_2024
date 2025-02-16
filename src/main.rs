use aoc_utils::Grid;
use std::fs::read_to_string;
use std::ops::AddAssign;

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

    let regions = regions
        .into_iter()
        .map(|r| {
            (
                r.0.into_iter()
                    .map(|x| x.0)
                    .collect::<Vec<(isize, isize)>>(),
                r.1,
            )
        })
        .collect::<Vec<(Vec<(isize, isize)>, char)>>();

    let plots = _input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // let mut checked = vec![vec![false; plots[0].len()]; plots.len()];
    let mut total_price: isize = 0;
    for region in regions {
        let mut left_sides = 0;
        let mut right_sides = 0;
        let mut top_sides = 0;
        let mut bot_sides = 0;
        for c in 0..plots[0].len() {
            let mut left_side = false;
            let mut right_side = false;
            for r in 0..plots.len() {
                if !region.0.contains(&(r as isize, c as isize)) {
                    continue;
                }
                if c == 0 || plots[r][c - 1] != region.1 {
                    if plots[r][c] == region.1 {
                        if !left_side {
                            left_sides += 1;
                        }
                        left_side = true;
                    } else {
                        left_side = false;
                    }
                }

                if c + 1 >= plots[0].len() || plots[r][c + 1] != region.1 {
                    if plots[r][c] == region.1 {
                        if !right_side {
                            right_sides += 1;
                        }
                        right_side = true;
                    } else {
                        right_side = false;
                    }
                }
            }
        }

        for r in 0..plots.len() {
            let mut top_side = false;
            let mut bot_side = false;
            for c in 0..plots[0].len() {
                if !region.0.contains(&(r as isize, c as isize)) {
                    continue;
                }
                if r == 0 || plots[r - 1][c] != region.1 {
                    if plots[r][c] == region.1 {
                        if !top_side {
                            top_sides += 1;
                        }
                        top_side = true;
                    } else {
                        top_side = false;
                    }
                }

                if r + 1 >= plots.len() || plots[r + 1][c] != region.1 {
                    if plots[r][c] == region.1 {
                        if !bot_side {
                            bot_sides += 1;
                        }
                        bot_side = true;
                    } else {
                        bot_side = false;
                    }
                }
            }
        }

        total_price += (left_sides + right_sides + top_sides + bot_sides) * region.0.len() as isize;
    }

    total_price
}

fn get_region(
    plots: &mut Grid<(char, bool)>,
    plot_type: char,
    position: (isize, isize),
    region_plots: &mut Vec<((isize, isize), usize)>,
) {
    if plots.item_ref(position.0, position.1).unwrap().0 == plot_type
        && !plots.item_ref(position.0, position.1).unwrap().1
    {
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
