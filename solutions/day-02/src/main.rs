use std::fs;

fn is_safe(level: &Vec<i32>) -> bool {
    let mut level_iter = level.iter();

    let mut prev = level_iter.next()
        .unwrap();

    let mut increasing: Option<bool> = None;

    for curr in level_iter {
        let diff = prev - curr;

        match diff {
            1 | 2 | 3 => {
                if let Some(true) = increasing {
                    return false;
                }
                increasing = Some(false);
            },
            -3 | -2 | -1 => {
                if let Some(false) = increasing {
                    return false;
                }
                increasing = Some(true);
            },
            _ => return false,
        }

        prev = curr;
    }

    true
}

fn is_safe_many(level: &Vec<i32>) -> bool {
    if is_safe(level) { return true; }

    let mut combs: Vec<Vec<i32>> = Vec::new();
    for i in 0..level.len() {
        let mut new_comb: Vec<i32> = Vec::new();

        new_comb.extend_from_slice(&level[..i]);
        new_comb.extend_from_slice(&level[i+1..]);

        combs.push(new_comb);
    }

    combs.iter().any(|x| is_safe(x))
}

fn main() {
    let input: String = fs::read_to_string("./input.in")
        .expect("No file found");

    let mut count1 = 0i32;
    let mut count2 = 0i32;
    for line in input.lines() {
        let level: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(&level) { count1 += 1 };
        if is_safe_many(&level) { count2 += 1 };
    }

    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
}
