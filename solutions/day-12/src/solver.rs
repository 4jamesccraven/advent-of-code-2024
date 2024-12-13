pub const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub const SURROUNDING: [(isize, isize); 8] = [
    (-1,  0),
    (-1, -1),
    ( 0, -1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
    ( 0,  1),
    (-1,  1),
];

pub struct Solver {
    pub map: Vec<Vec<char>>,
    pub rows: usize,
    pub cols: usize,
    visited: Vec<Vec<bool>>,
}

impl Solver {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        let rows = map.len();
        let cols = map[0].len();
        let visited = vec![vec![false; cols]; rows];

        Solver {
            map,
            rows,
            cols,
            visited,
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

    pub fn regions(&mut self) -> Vec<Vec<(usize, usize)>> {
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

    pub fn part_1(&mut self) -> usize {
        let answer = self.regions().iter()
            .map(|r| Self::area(r) * Self::perimeter(r))
            .sum();
        self.visited = vec![vec![false; self.cols]; self.rows];
        answer
    }

    pub fn part_2(&mut self) -> usize {
        let answer = self.regions().iter()
            .map(|r| Self::area(r) * Self::sides(r))
            .sum();
        self.visited = vec![vec![false; self.cols]; self.rows];
        answer
    }
}

