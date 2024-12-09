use std::fs::read_to_string;

fn main() {
    let inputs = vec![
        read_to_string("./orig.in").unwrap(),
        read_to_string("./input.in").unwrap()
    ];

    for (idx, input) in inputs.iter().enumerate() {
        if idx == 1 { break; }

        let mut file_system: Vec<Option<u32>> = Vec::new();

        let parsed_input = input.chars()
            .filter_map(|x| x.to_digit(10));

        let mut cycle = 0u32;
        let mut index = 0u32;
        for num in parsed_input {
            if cycle % 2 == 0 {
                for _ in 0..num {
                    file_system.push(Some(index));
                }
                index += 1;
            } else {
                for _ in 0..num {
                    file_system.push(None);
                }
            }
            cycle += 1;
        }

        let save_for_part2 = file_system.clone();

        while matches!(file_system.last(), Some(None)) {
            file_system.pop();
        }

        loop {
            let first_empty = file_system.iter()
                .enumerate()
                .filter(|(_, x)| x.is_none())
                .map(|(idx, _)| idx)
                .next();

            if let Some(idx) = first_empty {
                let end = file_system.len() - 1;
                file_system.swap(idx, end);
            }
            else {
                break;
            }

            while matches!(file_system.last(), Some(None)) {
                file_system.pop();
            }
        }

        let file_system: Vec<u32> = file_system.iter()
            .filter_map(|x| *x)
            .collect();

        let ans1: usize = file_system.iter()
            .enumerate()
            .map(|(i, v)| i * *v as usize)
            .sum();

        let filename = match idx {
            0 => "orig.in",
            1 => "input.in",
            _ => "",
        };

        println!("{filename}");
        println!("Part 1 {ans1}");
    }
}
