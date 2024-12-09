use std::fs;
use std::collections::HashSet;
use std::cmp::Ordering::{Less, Greater};

/*
 * This code was HEAVILY inspired by 
 * https://github.com/AndrejOrsula/aoc/blob/1cb5492c20cc89d988ab2bd562d83749e2611bd6/aoc2024/src/day5.rs
 * I wish I could take full credit, but I had no idea how to approach this problem. I do fully
 * undertand everything I wrote, and I also implemented everything without additional crates, but
 * I wouldn't have been able to do it without that as a reference.
 */

fn main() {
    let inputs = vec![
        fs::read_to_string("./orig.in").unwrap(),
        fs::read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let rules: HashSet<(u32, u32)> = input.lines()
            .take_while(|line| !line.is_empty())
            .filter_map(|line| {
                line.split_once("|")
                    .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
            })
            .collect();

        let updates: Vec<Vec<u32>> = input.lines()
            .skip(rules.len() + 1)
            .map(|line| {
                line.split(",")
                    .map(|x| x.parse().unwrap())
                    .collect()
            }).collect();

        let is_valid = |update: &Vec<u32>| -> bool {
            let mut nums = update.iter();
            let mut prev = nums.next().unwrap();

            for curr in nums {
                if rules.contains(&(*curr, *prev)) {
                    return false;
                }
                prev = curr;
            }

            true
        };

        let sort = |a: &u32, b: &u32| {
            if rules.contains(&(*b, *a)) {
                Less
            } else {
                Greater
            }
        };

        let ans1: u32 = updates.iter()
            .filter(|u| is_valid(u))
            .map(|u| u[u.len() / 2])
            .sum();

        let mut invalid: Vec<Vec<u32>> = updates.iter()
            .filter(|u| !is_valid(u))
            .map(|u| u.to_owned())
            .collect();

        invalid.iter_mut().for_each(|u| u.sort_by(sort));

        let ans2: u32 = invalid.iter()
            .map(|u| u[u.len() / 2])
            .sum();

        let file_name = String::from(
            match idx {
                0 => "orig.in",
                1 => "input.in",
                _ => "",
            });

        println!("{}", file_name);
        println!("Part 1: {}", ans1);
        println!("Part 2: {}", ans2);
    }
}
