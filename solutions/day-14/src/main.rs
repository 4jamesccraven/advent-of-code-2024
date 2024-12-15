use std::io;
use std::error::Error;
use std::fs::read_to_string;
use std::env::args;
use std::ops::Range;

#[derive(Debug, Clone)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
    field_sz: (isize, isize),
}

impl Robot {
    fn new(x: isize, y: isize, dx: isize, dy: isize, field: (isize, isize)) -> Self {
        Self {
            position: (x, y),
            velocity: (dx, dy),
            field_sz: field,
        }
    }

    fn step(&mut self) {
        let (x, y) = self.position;
        let (dx, dy) = self.velocity;
        let (x_max, y_max) = self.field_sz;

        let mut new_x = x + dx;
        let mut new_y = y + dy;

        loop {
            if new_x < 0 {
                new_x += x_max;
            }
            else if new_x >= x_max {
                new_x -= x_max;
            }
            else {
                break;
            }
        }

        loop {
            if new_y < 0 {
                new_y += y_max;
            }
            else if new_y >= y_max {
                new_y -= y_max;
            }
            else {
                break;
            }
        }

        self.position = (new_x, new_y);
    }
}

#[allow(dead_code)]
fn show_field(robots: &Vec<Robot>, field: (usize, usize)) {
    let (x_max, y_max) = field;
    for y in 0..y_max {
        for x in 0..x_max {
            let count: usize = robots
                .iter()
                .filter(|r| {
                    let (x_i, y_i) = r.position;

                    x_i == x as isize && y_i == y as isize
                })
                .count();

            if count != 0 {
                print!("{count}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn counts(robots: &Vec<Robot>, field: (usize, usize)) -> Vec<Vec<usize>> {
    let (x_max, y_max) = field;

    let mut counts: Vec<Vec<usize>> = vec![vec![0; x_max]; y_max];

    for y in 0..y_max {
        for x in 0..x_max {
            let count: usize = robots
                .iter()
                .filter(|r| {
                    let (x_i, y_i) = r.position;

                    x_i as usize == x && y_i as usize == y
                })
                .count();

            counts[y][x] = count;
        }
    }

    counts
}

fn part_1(robots: &Vec<Robot>, field: (usize, usize)) -> usize {
    let (x_max, y_max) = field;
    let (x_mid, y_mid) = (x_max / 2, y_max / 2);

    let counts: Vec<Vec<usize>> = counts(robots, field);

    let slices: [(Range<usize>, Range<usize>); 4] = [
        (0..x_mid, 0..y_mid),
        (0..x_mid, y_mid+1..y_max),
        (x_mid+1..x_max, 0..y_mid),
        (x_mid+1..x_max, y_mid+1..y_max),
    ];

    slices
        .iter()
        .map(|(cols, rows)| {
            counts[rows.clone()]
                .iter()
                .flat_map(|row| row[cols.clone()].iter().cloned())
                .sum::<usize>()
        })
        .product()
}

fn part_2(mut robots: Vec<Robot>, field: (usize, usize), progress: bool) -> usize {
    // We did 100 iterations by the time `part_2` takes ownership
    // of robots
    let mut idx = 101_usize;
    loop {
        for robot in robots.iter_mut() {
            robot.step();
        }

        let cnts = counts(&robots, field);

        let is_image = cnts
            .iter()
            .flatten()
            .all(|&x| x == 0 || x == 1);

        if progress {
            eprint!("\r{idx}");
        }

        if is_image {
            if progress {
                eprint!("\r               \r");
            }
            show_field(&robots, field);
            break;
        }
        idx += 1;
    }
    idx
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg = args()
        .skip(1)
        .next()
        .ok_or(
            io::Error::new(io::ErrorKind::InvalidInput, "usage: day-14 FILE")
        )?;

    const FIELD: (usize, usize) = (101, 103);

    let input = read_to_string(arg)?;

    let mut robots: Vec<_> = input
        .lines()
        .map(|line: &str| {
            let nums: Vec<isize> = line.split([' ', ',', '='])
                .filter_map(|num| num.parse::<isize>().ok())
                .collect();
            Robot::new(nums[0], nums[1], nums[2], nums[3], (FIELD.0 as isize, FIELD.1 as isize))
        })
        .collect();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.step();
        }
    }

    println!("Part 1: {}", part_1(&robots, FIELD));
    println!("Part 2: {}", part_2(robots, FIELD, false));

    Ok(())
}
