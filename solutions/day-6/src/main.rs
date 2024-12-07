use std::fs;
use std::collections::HashSet;

/*
 * This barely works. If you're seeking to employ me
 * and you see this, no you didn't.
 */


const DIRS: [(i32, i32); 4] = [
    ( 0,  1),
    ( 1,  0),
    ( 0, -1),
    (-1,  0),
];

#[derive(Clone)]
struct Solution {
    rows: usize,
    columns: usize,
    obstacles: Vec<(usize, usize)>,
    guard: (usize, usize),
    curr_dir: usize,
}

impl Solution {
    fn new(input: &str) -> Self {
         let rows = input.lines().count();
        let columns = input.lines().next().unwrap().chars().count();

        let mut obstacles: Vec<(usize, usize)> = Vec::new();
        let mut guard = (0, 0);
        let mut curr_dir: usize = 0;

        for (i, line) in input.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                match char {
                    dir @ ('^' | '<' | '>' | 'v') => {
                        guard = (i, j);
                        curr_dir = match dir {
                            '>' => 0,
                            'v' => 1,
                            '<' => 2,
                            '^' => 3,
                            _ => panic!(),
                        };
                    },
                    '#' => {
                        obstacles.push((i, j));
                    },
                    _ => {},
                }
            }
        }

        Solution {
            rows,
            columns,
            obstacles,
            guard,
            curr_dir,
        }
    }

    fn move_guard(&mut self) -> Option<(usize, usize)> {
        let (mut dy, mut dx) = DIRS[self.curr_dir % 4];
        let mut y = usize::try_from(self.guard.0 as i32 + dy).ok()?;
        let mut x = usize::try_from(self.guard.1 as i32 + dx).ok()?;

        while self.obstacles.contains(&(y, x)) {
            self.curr_dir += 1;
            (dy, dx) = DIRS[self.curr_dir % 4];
            y = usize::try_from(self.guard.0 as i32 + dy).ok()?;
            x = usize::try_from(self.guard.1 as i32 + dx).ok()?;
        }

        if y >= self.rows || x >= self.columns {
            return None;
        }

        self.guard = (y, x);

        Some((y, x))
    }
}

fn main() {
    let inputs = vec![
        fs::read_to_string("./orig.in").unwrap(),
        fs::read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let mut sol = Solution::new(input);

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while let Some(loc) = sol.move_guard() {
            visited.insert(loc);
        }

        let ans1 = visited.iter().count();

        let mut ans2 = 0usize;
        for place in visited {
            let mut sol_temp = Solution::new(input);
            let mut visited3: HashSet<(usize, usize, usize)> = HashSet::new();
            let mut first = true;
            sol_temp.obstacles.push(place);
            while let Some((y, x)) = sol_temp.move_guard() {
                let v3 = (sol_temp.curr_dir % 4, y, x);
                // print!("{:?}, ", v3);
                if !first && visited3.contains(&v3) {
                    ans2 += 1;
                    break;
                } else {
                    first = false;
                    visited3.insert(v3);
                }
            }
        }

        ans2 -= 1;

        let filename = match idx {
            0 => "orig.in",
            1 => "input.in",
            _ => "",
        };

        println!("{}", filename);
        println!("Part 1: {}", ans1);
        println!("Part 2: {}", ans2);
    }
}
