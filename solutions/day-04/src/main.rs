use std::fs;


fn main() {
    let inputs = vec![
        fs::read_to_string("orig.in").expect("orig.in not found"),
        fs::read_to_string("input.in").expect("input.in not found"),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let lines: Vec<Vec<char>> = input
            .lines()
            .map(|x| x.to_owned())
            .map(|x| x.chars().collect())
            .collect();

        let (rows, columns) = (lines.len(), lines[0].len());

        let word: [char; 4] = ['X', 'M', 'A', 'S'];

        let x_scale: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1,];
        let y_scale: [i32; 8] = [1, 0, -1, -1, 1, -1, 0, 1,];

        let x_mas_mask1: [i32; 5] = [0, 0, 1, 2, 2];
        let y_mas_mask: [i32; 5] = [0, 2, 1, 0, 2];

        let word2: Vec<&str> = vec!["MMASS", "MSAMS", "SMASM", "SSAMM"];

        let mut count_1 = 0u32;
        let mut count_2 = 0u32;
        for i in 0..rows {
            for j in 0..columns {
                if lines[i][j] == 'X' {
                    for dir in 0..8 {
                        let mut found = true;
                        let (x, y) = (x_scale[dir], y_scale[dir]);

                        for k in 1..4 {
                            let nx = i as i32 + (y*k);
                            let ny = j as i32 + (x*k);

                            if nx < 0 || nx >= rows as i32 {
                                found = false;
                                break;
                            }

                            if ny < 0 || ny >= columns as i32 {
                                found = false;
                                break;
                            }

                            if lines[nx as usize][ny as usize] != word[k as usize] {
                                found = false;
                                break;
                            }
                        }

                        if found { count_1 += 1 };
                    }
                }
                let mut r_word = String::new();
                for k in 0..5 {
                    let (x, y): (i32, i32) = (x_mas_mask1[k], y_mas_mask[k]);
                    let nx = i as i32 + y;
                    let ny = j as i32 + x;

                    if nx < 0 || nx >= rows as i32 {
                        break;
                    }

                    if ny < 0 || ny >= columns as i32 {
                        break;
                    }

                    r_word.push(lines[nx as usize][ny as usize]);
                }
                let r_good = word2.iter()
                    .any(|x| x == &r_word);

                if r_good { count_2 += 1 }
            }
        }

        match idx {
            0 => {
                println!("orig.in:");
            }
            1 => {
                println!("input.in");
            }
            _ => {}
        }

        println!("Part 1: {}", count_1);
        println!("Part 2: {}", count_2);
    }
}
