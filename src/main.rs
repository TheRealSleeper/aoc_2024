use aoc_utils::Grid;
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
    let grid: Grid<char> = _input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .into();
    let regions = get_regions(&grid);

    let mut total: isize = 0;
    for i in 0..regions.len() {
        let mut sum: isize = 0;
        for plant in regions[i].0.iter() {
            if !regions[i].0.contains(&(plant.0 - 1, plant.1)) {
                sum += 1;
            }
            if !regions[i].0.contains(&(plant.0 + 1, plant.1)) {
                sum += 1;
            }
            if !regions[i].0.contains(&(plant.0, plant.1 - 1)) {
                sum += 1;
            }
            if !regions[i].0.contains(&(plant.0, plant.1 + 1)) {
                sum += 1;
            }
        }
        let region_total = sum * regions[i].0.len() as isize;
        total += region_total;
    }

    total
}

fn part2(_input: &str) -> isize {
    let grid: Grid<char> = _input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>()
        .into();
    let regions = get_regions(&grid);

    let mut total_price: isize = 0;
    for region in regions {
        let mut left_sides = 0;
        let mut right_sides = 0;
        let mut top_sides = 0;
        let mut bottom_sides = 0;
        for c in 0..grid.dims().1 {
            let mut left_side = false;
            let mut right_side = false;
            for r in 0..grid.dims().0 {
                if !region.0.contains(&(r as isize, c as isize)) {
                    left_side = false;
                    right_side = false;
                    continue;
                }
                if (c == 0 || grid.item_ref(r, c - 1) != Some(&region.1))
                    && grid.item_ref(r, c) == Some(&region.1)
                {
                    if !left_side {
                        left_sides += 1;
                    }
                    left_side = true;
                } else {
                    left_side = false;
                }

                if (c + 1 >= grid.dims().1 || grid.item_ref(r, c + 1) != Some(&region.1))
                    && grid.item_ref(r, c) == Some(&region.1)
                {
                    if !right_side {
                        right_sides += 1;
                    }
                    right_side = true;
                } else {
                    right_side = false;
                }
            }
        }

        for r in 0..grid.dims().0 {
            let mut top_side = false;
            let mut bottom_side = false;
            for c in 0..grid.dims().1 {
                if !region.0.contains(&(r as isize, c as isize)) {
                    top_side = false;
                    bottom_side = false;
                    continue;
                }
                if (r == 0 || grid.item_ref(r - 1, c) != Some(&region.1)) && grid.item_ref(r, c) == Some(&region.1) {
                    if !top_side {
                        top_sides += 1;
                    }
                    top_side = true;
                } else {
                    top_side = false;
                }

                if (r + 1 >= grid.dims().1 || grid.item_ref(r + 1, c) != Some(&region.1)) && grid.item_ref(r, c) == Some(&region.1) {
                    if !bottom_side {
                        bottom_sides += 1;
                    }
                    bottom_side = true;
                } else {
                    bottom_side = false;
                }
            }
        }

        let region_total = (left_sides + right_sides + top_sides + bottom_sides) * region.0.len();
        total_price += region_total as isize;
    }

    total_price
}

fn get_regions(grid: &Grid<char>) -> Vec<(Vec<(isize, isize)>, char)> {
    let mut regions: Vec<(Vec<(isize, isize)>, char)> = Vec::new();
    let mut checked: Grid<bool> = Grid::from(vec![
        vec![false; grid.dims().1 as usize];
        grid.dims().0 as usize
    ]);
    let mut added: Option<char> = None;

    'search: loop {
        let mut current_type = '-';
        if let Some(c) = added {
            current_type = c;
            added = None;
        }
        let mut region = Vec::<(isize, isize)>::new();
        if current_type != '-' {
            region = regions.pop().unwrap().0;
        }
        for r in 0..grid.dims().0 {
            for c in 0..grid.dims().1 {
                if current_type == '-' && !*checked.item_ref(r, c).unwrap() {
                    current_type = *grid.item_ref(r, c).unwrap();
                    region.push((r, c));
                    checked.item_set(r, c, true);
                    continue;
                } else if current_type == '-' || *checked.item_ref(r, c).unwrap() {
                    continue;
                }
                if region.is_empty()
                    && grid.item_ref(r, c) == Some(&current_type)
                    && !*checked.item_ref(r, c).unwrap()
                {
                    region.push((r, c));
                    checked.item_set(r, c, true);
                } else if grid.item_ref(r, c).unwrap() == &current_type {
                    let current_position = grid.traverse((r, c));
                    if current_position.item_ref_up() == Some(&current_type)
                        && checked.item_ref(r - 1, c) == Some(&true)
                        || current_position.item_ref_down() == Some(&current_type)
                            && checked.item_ref(r + 1, c) == Some(&true)
                        || current_position.item_ref_left() == Some(&current_type)
                            && checked.item_ref(r, c - 1) == Some(&true)
                        || current_position.item_ref_right() == Some(&current_type)
                            && checked.item_ref(r, c + 1) == Some(&true)
                    {
                        checked.item_set(r, c, true);
                        region.push((r, c));
                        if added.is_none() {
                            added = Some(current_type);
                        }
                    }
                }
            }
        }
        if current_type == '-' {
            break 'search;
        } else {
            if !region.is_empty() {
                regions.push((region, current_type));
            }
        }
    }

    regions
}
