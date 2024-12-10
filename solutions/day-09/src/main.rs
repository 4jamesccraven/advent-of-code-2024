use std::fs::read_to_string;

/*
 * I am not proud of this code, I am just glad I got my answer.
 * Make sure to run with cargo run --release, otherwise it is very
 * slow.
 */

fn main() {
    let inputs = vec![
        read_to_string("./orig.in").unwrap(),
        read_to_string("./input.in").unwrap()
    ];

    for (idx, input) in inputs.iter().enumerate() {
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

        let mut file_system: Vec<(usize, Option<usize>)> = input.chars()
            .filter_map(|x| x.to_digit(10))
            .enumerate()
            .map(|(idx, x)| {
                let block = if idx % 2 == 0 {
                    Some(idx / 2)
                } else {
                    None
                };
                (x as usize, block)
            })
            .collect();

        let mut visited: Vec<usize> = Vec::new();
        loop {
            let mut merged = Vec::new();
            let mut merge_iter = file_system.iter();

            let mut prev = merge_iter.next().unwrap();
            merged.push(*prev);
            for curr in merge_iter {
                if prev.1.is_none() && prev.1 == curr.1 {
                    let mut merged_elem: (usize, Option<usize>) = merged.pop().unwrap();
                    merged_elem.0 += curr.0;
                    merged.push(merged_elem);
                } else {
                    merged.push(*curr);
                }
                prev = curr;
            }
            file_system = merged;

            file_system.retain(|(x, _)| *x != 0);

            // for (qty, _) in &file_system {
            //     eprint!("{}{}", qty, String::from(" ").repeat(qty - 1));
            // }
            // eprintln!("|");

            // for (qty, val) in &file_system {
            //     for _ in 0..*qty {
            //         if let Some(num) = val {
            //             eprint!("{num}");
            //         } else {
            //             eprint!(".");
            //         }
            //     }
            // }
            // eprintln!("|");

            let (old_idx, size_needed, block_no): (usize, usize, usize) = file_system.iter()
                .enumerate()
                .rev()
                .find_map(|(idx, (size_needed, block_no))| {
                    if !visited.contains(&(*block_no)?) {
                        Some((idx, *size_needed, (*block_no)?))
                    } else {
                        None
                    }
                })
                .unwrap();

            visited.push(block_no);

            // eprintln!("Finding a group of size >= {size_needed} for {block_no}...");

            let new_slot: Option<(usize, usize)> = file_system.iter()
                .enumerate()
                .find_map(|(idx, (size, val))| {
                    if *size >= size_needed && val.is_none() {
                        Some((idx, *size))
                    } else {
                        None
                    }
                });

            if let Some((new_idx, new_size)) = new_slot {
                // eprintln!("Found a group of size {new_size} at {new_idx}");
                if new_idx < old_idx {
                    if new_size == size_needed {
                        file_system.swap(new_idx, old_idx);
                    } else {
                        let diff = new_size - size_needed;
                        file_system.swap(new_idx, old_idx);
                        file_system.insert(new_idx + 1, (diff, None));
                        file_system[old_idx + 1].0 = size_needed;
                    }
                }
            }


            if block_no == 0 {
                if idx == 1 {
                    eprint!("\r{}\r", " ".repeat(52));
                }
                break;
            } else if idx == 1 {
                let num_equal = (50.0 * ((10_000 - block_no) as f64 / 10_000.0)) as usize;
                eprint!("\r[{}{}]", String::from("=").repeat(num_equal),
                    String::from("-").repeat(50 - num_equal));
            }
        }


        let ans2: usize = file_system.iter()
            .flat_map(|(qty, val)| {
                vec![val].repeat(*qty)
            })
            .enumerate()
            .filter_map(|(idx, val)| {
                if val.is_some() {
                    Some((idx * (*val)?) as usize)
                } else {
                    None
                }
            })
            .sum();

        let filename = match idx {
            0 => "orig.in",
            1 => "input.in",
            _ => "",
        };

        println!("{filename}");
        println!("Part 1 {ans1}");
        println!("Part 2 {ans2}");
    }
}
