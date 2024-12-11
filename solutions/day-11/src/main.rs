use std::fs::read_to_string;
use std::collections::HashMap;

fn process_stone(stone: usize, depth: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&result) = cache.get(&(stone, depth)) {
        return result;
    }

    if depth == 0 {
        cache.insert((stone, depth), 1);
        return 1;
    }
    if stone == 0 {
        let to_cache = process_stone(1, depth - 1, cache);
        cache.insert((stone, depth), to_cache);
        return to_cache;
    }

    let stone_str = stone.to_string();
    let len = stone_str.len();
    if len % 2 == 1 {
        let to_cache = process_stone(stone * 2024, depth - 1, cache);
        cache.insert((stone, depth), to_cache);
        return to_cache;
    }

    let split: [usize; 2] = [stone_str[..len/2].parse().unwrap(),
        stone_str[len/2..].parse().unwrap()];

    let to_cache = split.iter()
        .map(|&x| process_stone(x, depth - 1, cache))
        .sum();

    cache.insert((stone, depth), to_cache);

    to_cache
}

fn main() {
    let inputs = vec![
        read_to_string("./orig.in").unwrap(),
        read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        // if idx == 1 { break; }

        let stones: Vec<usize> = input.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let mut cache = HashMap::new();

        let ans1: usize = stones.iter()
            .map(|&stone| process_stone(stone, 25, &mut cache))
            .sum();

        let ans2: usize = stones.iter()
            .map(|&stone| process_stone(stone, 75, &mut cache))
            .sum();

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
