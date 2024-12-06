use std::fs;
use std::collections::HashSet;

fn main() {
    let inputs = vec![
        fs::read_to_string("./orig.in").unwrap(),
        fs::read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let directions: [(i32, i32); 4] = [
            ( 0,  1),
            ( 1,  0),
            ( 0, -1),
            (-1,  0),
        ];

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

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut move_guard = || -> Option<(usize, usize)> {
            let (mut dy, mut dx) = directions[curr_dir % 4];
            let mut y = usize::try_from(guard.0 as i32 + dy).ok()?;
            let mut x = usize::try_from(guard.1 as i32 + dx).ok()?;

            while obstacles.contains(&(y, x)) {
                curr_dir += 1;
                (dy, dx) = directions[curr_dir % 4];
                y = usize::try_from(guard.0 as i32 + dy).ok()?;
                x = usize::try_from(guard.1 as i32 + dx).ok()?;
            }

            if y >= rows || x >= columns {
                return None;
            }

            guard = (y, x);

            Some((y, x))
        };

        while let Some(loc) = move_guard() {
            visited.insert(loc);
        }

        let ans1 = visited.iter().count();

        let filename = match idx {
            0 => "orig.in",
            1 => "input.in",
            _ => "",
        };

        println!("{}", filename);
        println!("{}", ans1);
    }
}
