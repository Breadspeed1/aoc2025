pub fn run() -> i64 {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();

    let stuff = input
        .lines()
        .map(|l| {
            let nums: Vec<i64> = l
                .chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect();
            let mut ns: Vec<i64> = vec![];
            let mut last_idx = 0;

            for i in (0..12).rev() {
                let nums = &nums[(last_idx)..(nums.len() - i)];

                let max = nums.iter().max().unwrap();
                let max_idx = nums.iter().position(|v| v == max).unwrap();

                ns.push(*max);
                last_idx = max_idx + last_idx + 1;
            }

            let mut out = 0;

            for (i, n) in ns.iter().enumerate() {
                out += n * 10_i64.pow(12 - 1 - i as u32);
            }

            out
        })
        .sum();

    stuff
}
