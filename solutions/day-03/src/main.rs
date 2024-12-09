use std::fs;


fn main() {
    let input = fs::read_to_string("./input.in")
        .expect("File not found");

    let mut nums: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();

    let mut r#do: bool = true;

    for i in 0..(input.len() - 7) {
        if &input[i..i+4] == "mul(" {
            let parenthesis = input[i+4..].find(')');

            if let Some(end) = parenthesis {
                let theoretical_numbers: Vec<&str> = input[i+4..i+4+end]
                    .split(',')
                    .collect();

                let length_good = theoretical_numbers.len() == 2;
                let valid_nums = theoretical_numbers.iter()
                    .map(|x| x.parse::<i32>())
                    .all(|x| x.is_ok());

                if length_good && valid_nums {
                    let next_val: i32 = theoretical_numbers.iter()
                        .map(|x| x.parse::<i32>().unwrap())
                        .product();

                    nums.push(next_val);
                    if r#do {
                        nums2.push(next_val);
                    }
                }
            }
        } else if &input[i..i+4] == "do()" {
            r#do = true;
        } else if &input[i..i+7] == "don't()" {
            r#do = false;
        }
    }

    let ans1: i32 = nums.iter().sum();
    let ans2: i32 = nums2.iter().sum();

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
