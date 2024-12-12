use std::fs::read_to_string;

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

const SURROUNDING: [(isize, isize); 8] = [
    (-1,  0),
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
    ( 0,  1),
    (-1,  1),
];

struct Solver {
    map: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl Solver {
    fn new(map: Vec<Vec<char>>) -> Self {
        let rows = map.len();
        let cols = map[0].len();
        let visited = vec![vec![false; cols]; rows];

        Solver {
            map,
            visited,
            rows,
            cols,
        }
    }

    fn find_region(&mut self, row: usize, col: usize, region: char) -> Vec<(usize, usize)> {
        self.visited[row][col] = true;
        let xb = col as isize;
        let yb = row as isize;
        let (rows, cols) = (self.rows, self.cols);

        let in_bounds = |(x, y): (isize, isize)| {
            x >= 0 &&
            x < cols as isize &&
            y >= 0 &&
            y < rows as isize
        };

        let mut r_val = vec![(col, row)];

        let next_elems: Vec<_> = DIRECTIONS.iter()
            .map(|(x, y)| (xb + x, yb + y))
            .filter(|(x, y)| in_bounds((*x, *y)))
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|(x, y)| {
                !self.visited[*y][*x]
                && self.map[*y][*x] == region
            })
            .collect();

        next_elems.iter()
            .for_each(|(x, y)| {
                r_val.extend(self.find_region(*y, *x, region));
            });

        r_val
    }

    fn regions(&mut self) -> Vec<Vec<(usize, usize)>> {
        let mut r_val = Vec::new();
        let map_view = self.map.clone();

        for (i, line) in map_view.iter().enumerate() {
            for (j, &ch) in line.iter().enumerate() {
                if !self.visited[i][j] {
                    let region = self.find_region(i, j, ch);
                    if region.len() != 0 {
                        r_val.push(region);
                    }
                }
            }
        }

        r_val.iter_mut()
            .for_each(|r| {
                r.sort();
                r.dedup();
            });

        r_val
    }

    fn area(region: &Vec<(usize, usize)>) -> usize {
        region.len()
    }

    fn perimeter(region: &Vec<(usize, usize)>) -> usize {
        region.iter()
            .map(|(x, y)| (*x as isize, *y as isize))
            .map(|(x, y)| {
                DIRECTIONS.iter()
                    .filter(|(x_i, y_i)| {
                        !region.contains(&((x + x_i) as usize, (y + y_i) as usize))
                    })
                    .count()
            })
            .sum()
    }

    fn corners(region: &Vec<(usize, usize)>) -> usize {
        let surroundings: Vec<Vec<_>> = region.iter()
            .map(|(x, y)| (*x as isize, *y as isize))
            .map(|(x, y)| {
                SURROUNDING.iter()
                    .map(|(x_i, y_i)| {
                        region.contains(&((x + x_i) as usize, (y + y_i) as usize))
                    })
                    .collect()
            })
            .collect();

        let elbows: Vec<(usize, usize, usize)> = vec![
            (0, 1, 2),
            (2, 3, 4),
            (4, 5, 6),
            (6, 7, 0),
        ];

        let mut accumulator = 0_usize;
        for surrounding in surroundings {
            accumulator += elbows.iter()
                .filter(|(a, b, c)| {
                    surrounding[*a]
                    && !surrounding[*b]
                    && surrounding[*c]
                })
                .count();

            accumulator += elbows.iter()
                .filter(|(a, b, c)| {
                    !surrounding[*a]
                    && !surrounding[*b]
                    && !surrounding[*c]
                })
                .count();

            accumulator += elbows.iter()
                .filter(|(a, b, c)| {
                    !surrounding[*a]
                    && surrounding[*b]
                    && !surrounding[*c]
                })
                .count()
        }

        accumulator
    }

    fn sides(region: &Vec<(usize, usize)>) -> usize {
        Self::corners(region)
    }

    fn part_1(&mut self) -> usize {
        let answer = self.regions().iter()
            .map(|r| Self::area(r) * Self::perimeter(r))
            .sum();
        self.visited = vec![vec![false; self.cols]; self.rows];
        answer
    }

    fn part_2(&mut self) -> usize {
        let answer = self.regions().iter()
            .map(|r| Self::area(r) * Self::sides(r))
            .sum();
        self.visited = vec![vec![false; self.cols]; self.rows];
        answer
    }
}

fn main() {
    let inputs = vec![
        read_to_string("./orig.in").unwrap(),
        read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let map: Vec<Vec<_>> = input.lines()
            .map(|x| x.chars().collect())
            .collect();

        let mut solver = Solver::new(map);

        let ans1: usize = solver.part_1();
        let ans2: usize = solver.part_2();

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


#[cfg(test)]
mod test {
    use crate::Solver;
    use std::fs::read_to_string;

    #[test]
    fn part_1_1() {
        let input1 = read_to_string("1.in").unwrap();

        let test1_map = input1.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test1 = Solver::new(test1_map);

        assert_eq!(test1.part_1(), 140);
    }

    #[test]
    fn part_1_2() {
        let input2 = read_to_string("2.in").unwrap();

        let test2_map = input2.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test2 = Solver::new(test2_map);

        assert_eq!(test2.part_1(), 772);
    }

    #[test]
    fn part_2_2()  {
        let input2 = read_to_string("2.in").unwrap();

        let test2_map = input2.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test2 = Solver::new(test2_map);

        assert_eq!(test2.part_2(), 436);
    }

    #[test]
    fn part_2_3() {
        let input3 = read_to_string("3.in").unwrap();

        let test3_map = input3.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test3 = Solver::new(test3_map);

        assert_eq!(test3.part_2(), 236);
    }

    #[test]
    fn part_2_4() {
        let input4 = read_to_string("4.in").unwrap();

        let test4_map = input4.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test4 = Solver::new(test4_map);

        assert_eq!(test4.part_2(), 368);
    }
}
