use std::fs;

fn main() {
    let inputs = vec![
        fs::read_to_string("./orig.in").unwrap(),
        fs::read_to_string("./input.in").unwrap(),
    ];

    for (idx, input) in inputs.iter().enumerate() {
        let eqs: Vec<Vec<_>> = input.lines()
            .map(|l| l.replace(":", ""))
            .map(|l| l.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect())
            .collect();

        let mut ans1 = 0usize;
        let mut ans2 = 0usize;

        for nums in eqs {
            let target = nums[0];
            let mut possible: Vec<usize> = vec![nums[1]];
            let mut possible2: Vec<usize> = vec![nums[1]];

            for num in &nums[2..] {
                possible = possible.iter()
                    .flat_map(|p| vec![num*p, num+p])
                    .collect();
                possible2 = possible2.iter()
                    .flat_map(|p| {
                        vec![
                            num*p,
                            num+p,
                            format!("{}{}", p, num)
                                .parse::<usize>().unwrap(),
                        ]})
                    .collect();
            }

            if possible.contains(&target) {
                ans1 += target;
            }
            if possible2.contains(&target) {
                ans2 += target;
            }
        }

        let filename = match idx {
            0 => "orig.in",
            1 => "orig.in",
            _ => "",
        };

        println!("{}", filename);
        println!("Part 1: {}", ans1);
        println!("Part 2: {}", ans2);
    }
}
