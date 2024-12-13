pub mod solver;

use std::fs::read_to_string;
use solver::Solver;

fn main() {
    let inputs = vec![
        read_to_string("./orig.in").unwrap(),
        read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let map: Vec<Vec<_>> = input.lines()
            .map(|x| x.chars().collect())
            .collect();

        let mut solver = Solver::new(map);

        let ans1: usize = solver.part_1();
        let ans2: usize = solver.part_2();

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


#[cfg(test)]
mod test {
    use crate::Solver;
    use std::fs::read_to_string;

    #[test]
    fn part_1_1() {
        let input1 = read_to_string("1.in").unwrap();

        let test1_map = input1.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test1 = Solver::new(test1_map);

        assert_eq!(test1.part_1(), 140);
    }

    #[test]
    fn part_1_2() {
        let input2 = read_to_string("2.in").unwrap();

        let test2_map = input2.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test2 = Solver::new(test2_map);

        assert_eq!(test2.part_1(), 772);
    }

    #[test]
    fn part_2_2()  {
        let input2 = read_to_string("2.in").unwrap();

        let test2_map = input2.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test2 = Solver::new(test2_map);

        assert_eq!(test2.part_2(), 436);
    }

    #[test]
    fn part_2_3() {
        let input3 = read_to_string("3.in").unwrap();

        let test3_map = input3.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test3 = Solver::new(test3_map);

        assert_eq!(test3.part_2(), 236);
    }

    #[test]
    fn part_2_4() {
        let input4 = read_to_string("4.in").unwrap();

        let test4_map = input4.lines()
            .map(|v| v.chars().collect())
            .collect();

        let mut test4 = Solver::new(test4_map);

        assert_eq!(test4.part_2(), 368);
    }
}
