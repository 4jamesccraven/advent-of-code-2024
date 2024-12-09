use std::fs;
use std::collections::HashMap;


fn main() {
    let input = fs::read_to_string("./input.in")
        .expect("input file not found");

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let vals: Vec<i32> = line.split_whitespace()
            .map(|val| String::from(val))
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        list1.push(vals[0]);
        list2.push(vals[1]);
    }

    list1.sort();
    list2.sort();

    let answer: i32 = list1.iter()
        .zip(list2.iter())
        .map(|(x, y)| (x - y).abs() )
        .sum();

    println!("Part 1: {}", answer);

    let mut counter: HashMap<i32, i32> = HashMap::new();

    for num in list2.iter() {
        *counter.entry(*num).or_insert(0) += 1;
    }

    let answer: i32 = list1.iter()
        .map(|x| x * *counter.entry(*x).or_insert(0))
        .sum();

    println!("Part 2: {}", answer);
}
