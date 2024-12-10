use std::fs::read_to_string;
use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone)]
struct Scorer {
    map: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl Scorer {
    fn new(map: Vec<Vec<u32>>) -> Self {
        let rows = map.len();
        let cols = map[0].len();
        Scorer {
            map,
            rows,
            cols,
        }
    }

    fn score(&self, x: usize, y: usize, curr_depth: u32) -> HashSet<(usize, usize)> {
        let mut r_val: HashSet<(usize, usize)> = HashSet::new();
        if self.map[y][x] == 9 && curr_depth == 9 {
            r_val.insert((x, y));
            return r_val;
        }
        else if self.map[y][x] != curr_depth {
            return r_val;
        }
        else {
            let x_b = x as isize;
            let y_b = y as isize;
            let in_bounds = |(x, y): (isize, isize)| {
                x >= 0 &&
                x < self.cols as isize &&
                y >= 0 &&
                y < self.rows as isize
            };

            let next_elems: Vec<(usize, usize)> = DIRECTIONS.iter()
                .map(|(x, y)| (x_b + x, y_b + y))
                .filter(|idx| in_bounds(*idx))
                .map(|(x, y)| (x as usize, y as usize))
                .collect();

            next_elems.iter()
                .for_each(|(x, y)| {
                    r_val.extend(self.score(*x, *y, curr_depth + 1));
                });

            return r_val;
        }
    }

    fn score_all(&self) -> usize {
        self.map.iter()
            .enumerate()
            .flat_map(|(row, x)| {
                x.iter()
                    .enumerate()
                    .map(|(col, val)| (col, row, val))
                    .collect::<Vec<_>>()
            })
            .filter(|(_, _, &val)| val == 0)
            .map(|(x, y, _)| self.score(x, y, 0).len())
            .sum()
    }

    fn rate(&self, x: usize, y: usize, curr_depth: u32) -> usize {
        if self.map[y][x] == 9 && curr_depth == 9 {
            return 1;
        }
        else if self.map[y][x] != curr_depth {
            return 0;
        }
        else {
            let x_b = x as isize;
            let y_b = y as isize;
            let in_bounds = |(x, y): (isize, isize)| {
                x >= 0 &&
                x < self.cols as isize &&
                y >= 0 &&
                y < self.rows as isize
            };

            let next_elems: Vec<(usize, usize)> = DIRECTIONS.iter()
                .map(|(x, y)| (x_b + x, y_b + y))
                .filter(|idx| in_bounds(*idx))
                .map(|(x, y)| (x as usize, y as usize))
                .collect();

            let r_val: usize = next_elems.iter()
                .map(|(x, y)| self.rate(*x, *y, curr_depth + 1))
                .sum();

            return r_val;
        }
    }

    fn rate_all(&self) -> usize {
        self.map.iter()
            .enumerate()
            .flat_map(|(row, x)| {
                x.iter()
                    .enumerate()
                    .map(|(col, val)| (col, row, val))
                    .collect::<Vec<_>>()
            })
            .filter(|(_, _, &val)| val == 0)
            .map(|(x, y, _)| self.rate(x, y, 0))
            .sum()
    }
}

fn main() {
    let inputs = vec![
        read_to_string("./orig.in").unwrap(),
        read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let map: Vec<Vec<_>> = input.lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect();

        let map_scorer = Scorer::new(map);

        let ans1: usize = map_scorer.score_all();

        let ans2: usize = map_scorer.rate_all();

        let file_name = match idx {
            0 => "orig.in",
            1 => "input.in",
            _ => "",
        };

        println!("{file_name}");
        println!("Part 1: {ans1}");
        println!("Part 2: {ans2}");
    }
}
