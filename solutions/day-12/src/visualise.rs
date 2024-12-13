#[allow(dead_code)]
mod solver;

use solver::{Solver, SURROUNDING};
use std::env::args;
use std::path::Path;
use std::fs::read_to_string;
use std::collections::HashMap;

const COLOURS: [&str; 8] = ["\x1b[90m", "\x1b[91m", "\x1b[92m", "\x1b[94m", "\x1b[93m", "\x1b[95m", "\x1b[96m", "\x1b[97m"];

fn get_map() -> Vec<Vec<char>> {
    let args: Vec<_> = args().skip(1)
        .collect();

    if let None = args.get(0) {
        eprintln!("usage: visualise FILE");
        std::process::exit(1);
    }

    let file = Path::new(&args[0])
        .try_exists();

    if let Err(_) = file {
        eprintln!("{} is not a valid file", args[0]);
    }

    read_to_string(&args[0])
        .expect("File was verified to exist")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn get_regions(map: &Vec<Vec<char>>) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut solver = Solver::new(map.clone());
    solver.regions()
        .iter()
        .enumerate()
        .map(|(k, v)| (k, v.to_owned()))
        .collect()
}

fn main() {
    let map = get_map();
    let regions = get_regions(&map);
    let regions_inv: HashMap<_, _> = regions.iter()
        .flat_map(|(reg, coords)| {
            coords.iter()
                .map(|c| (c.clone(), reg.clone()))
        })
        .collect();
    let region_count = regions.len();

    let mut adj = vec![vec![false; region_count]; region_count];
    for (k, v) in &regions {
        for (x, y) in v {
            let xb = *x as isize;
            let yb = *y as isize;
            let surr: Vec<_> = SURROUNDING.iter()
                .map(|(x, y)| ((xb + *x) as usize, (yb + *y) as usize))
                .collect();

            for idx_s in surr {
                let adj_region = regions_inv.get(&idx_s);

                if let Some(reg) = adj_region {
                    adj[*reg][*k] = true;
                    adj[*k][*reg] = true;
                }
            }
        }
    }

    let mut result = vec![usize::MAX; region_count];
    let mut available = vec![false; region_count];

    result[0] = 0;
    for u in 1..region_count {
        for v in 0..region_count {
            if adj[u][v] && result[v] != usize::MAX {
                available[result[v]] = true;
            }
        }

        let mut colour = 0;
        while colour < region_count && available[colour] {
            colour += 1;
        }

        result[u] = colour;

        available = vec![false; region_count];
    }

    let info: HashMap<(usize, usize), &str> = regions_inv.iter()
        .map(|(k, reg)| (*k, COLOURS[result[*reg]]))
        .collect();

    for (row, line) in map.iter().enumerate() {
        for (col, val) in line.iter().enumerate() {
            let to_print = format!("{}{}\x1b[0m", info[&(col, row)], val);
            print!("{to_print}");
        }
        println!();
    }
}
