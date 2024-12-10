use std::fs::read_to_string;
use std::collections::HashSet;

fn map_lines<'a>(row: usize, line: &'a str) -> impl Iterator<Item = (i32, i32, char)> + 'a {
    line.chars()
        .enumerate()
        .filter(|(_, char)| char.is_digit(10) || char.is_ascii_alphabetic())
        .map(move |(col, char)| (col as i32, row as i32, char))
}

fn main() {
    let inputs = vec![
        read_to_string("./orig.in").unwrap(),
        read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let parsed_file: Vec<(i32, i32, char)> = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| map_lines(row, line))
            .collect();

        let p_iter = parsed_file.iter();

        let mut antinodes1: HashSet<(i32, i32)> = HashSet::new();
        let mut antinodes2: HashSet<(i32, i32)> = HashSet::new();

        let rows = input.lines().count() as i32;
        let cols = input.lines().next().unwrap().len() as i32;

        for (x_1, y_1, f_1) in p_iter.clone() {
            p_iter.clone()
                .filter(|(x_2, y_2, _)| !(x_1 == x_2 && y_1 == y_2))
                .filter(|(_, _, f_2)| f_2 == f_1)
                .for_each(|(x_2, y_2, _)| {
                    let slope: (i32, i32) = (x_2 - x_1, y_2 - y_1);

                    let mut for_ans1 = true;
                    let mut step = 1i32;
                    loop {
                        let a1 = (x_1 - slope.0 * step, y_1 - slope.1 * step);

                        if (a1.0 >= 0 && a1.0 < cols) && (a1.1 >= 0 && a1.1 < rows) {
                            if for_ans1 {
                                antinodes1.insert(a1);
                                for_ans1 = false;
                            }
                            antinodes2.insert(a1);

                            step +=1;
                        } else { break; }
                    }
                    step = 1i32;
                    loop {
                        let a2 = (x_1 + slope.0 * step, y_1 + slope.1 * step);

                        for_ans1 = (x_2 + slope.0 == a2.0)
                            && (y_2 + slope.1 == a2.0);

                        if (a2.0 >= 0 && a2.0 < cols) && (a2.1 >= 0 && a2.1 < rows) {
                            if for_ans1 {
                                antinodes1.insert(a2);
                            }
                            antinodes2.insert(a2);

                            step += 1;
                        } else { break; }
                    }
                });
        }

        /*  for i in 0..rows {
         *      for j in 0..cols {
         *          if antinodes2.contains(&(j, i)) {
         *              eprint!("#");
         *          } else {
         *              eprint!(".");
         *          }
         *      }
         *      eprint!("\n");
         *  }
         */

        let ans1 = antinodes1.iter().count();
        let ans2 = antinodes2.iter().count();

        let file_name = match idx {
            0 => "./orig.in",
            1 => "./input.in",
            _ => "",
        };

        println!("{}", file_name);
        println!("Part 1: {}", ans1);
        println!("Part 2: {}", ans2);
    }
}
