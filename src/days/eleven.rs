use itertools::Itertools;
use std::usize;

pub fn run(input: String) {
    let galaxies = get_galaxies(input);
    let x_max = galaxies.iter().fold(0, |acc, x| acc.max(x.0));
    let y_max = galaxies.iter().fold(0, |acc, y| acc.max(y.1));
    let mut new_galaxies = galaxies.clone();

    let mut expanded = 0;
    for x in 0..=x_max {
        if galaxies.iter().all(|g| g.0 != x) {
            expand_galaxies(&mut new_galaxies, x + expanded, true, 1);
            expanded += 1;
        }
    }
    expanded = 0;
    for y in 0..=y_max {
        if galaxies.iter().all(|g| g.1 != y) {
            expand_galaxies(&mut new_galaxies, y + expanded, false, 1);
            expanded += 1;
        }
    }
    let mut total = 0;
    for galaxy in new_galaxies.iter().combinations(2) {
        total += galaxy[0].0.abs_diff(galaxy[1].0);
        total += galaxy[0].1.abs_diff(galaxy[1].1);
    }
    let x_max = new_galaxies.iter().fold(0, |acc, x| acc.max(x.0));
    let y_max = new_galaxies.iter().fold(0, |acc, y| acc.max(y.1));
    for y in 0..=y_max {
        for x in 0..=x_max {
            if new_galaxies.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!("{}", total);
}

pub fn runtwo(input: String) {
    let galaxies = get_galaxies(input);
    let x_max = galaxies.iter().fold(0, |acc, x| acc.max(x.0));
    let y_max = galaxies.iter().fold(0, |acc, y| acc.max(y.1));
    let mut new_galaxies = galaxies.clone();

    let mut expanded = 0;
    for x in 0..=x_max {
        if galaxies.iter().all(|g| g.0 != x) {
            expand_galaxies(&mut new_galaxies, x + expanded, true, 999999);
            expanded += 999999;
        }
    }
    expanded = 0;
    for y in 0..=y_max {
        if galaxies.iter().all(|g| g.1 != y) {
            expand_galaxies(&mut new_galaxies, y + expanded, false, 999999);
            expanded += 999999;
        }
    }
    let mut total = 0;
    for galaxy in new_galaxies.iter().combinations(2) {
        total += galaxy[0].0.abs_diff(galaxy[1].0);
        total += galaxy[0].1.abs_diff(galaxy[1].1);
    }
    let x_max = new_galaxies.iter().fold(0, |acc, x| acc.max(x.0));
    let y_max = new_galaxies.iter().fold(0, |acc, y| acc.max(y.1));
    // for y in 0..=y_max {
    //     for x in 0..=x_max {
    //         if new_galaxies.contains(&(x, y)) {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!()
    // }
    println!("{}", total);
}

fn get_galaxies(input: String) -> Vec<(usize, usize)> {
    let lines = input.as_str().lines();
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                result.push((x, y));
            }
        }
    }
    result
}

fn expand_galaxies(galaxies: &mut Vec<(usize, usize)>, target: usize, row: bool, size: usize) {
    for galaxy in galaxies {
        if row {
            if galaxy.0 > target {
                galaxy.0 += size;
            }
        } else if galaxy.1 > target {
            galaxy.1 += size;
        }
    }
}
