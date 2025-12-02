pub fn run() -> i64 {
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();

    let ranges: Vec<(i64, i64)> = input
        .split(",")
        .map(|seg| {
            let vals = seg
                .split("-")
                .map(|v| {
                    v.trim()
                        .parse::<i64>()
                        .inspect_err(|e| println!("{e}, {v}"))
                        .unwrap()
                })
                .collect::<Vec<i64>>();

            (vals[0], vals[1])
        })
        .collect();

    let mut total = 0;

    for (start, end) in ranges {
        for i in start..=end {
            let digits = i.ilog10() + 1;
            let dig2 = digits / 2;
            for d in (1..=dig2).filter(|d| digits % d == 0) {
                let mut v = i;

                for seg in 0..(digits / d) {
                    //println!("{d}, {v}");
                    v -= 10_i64.pow(d * seg) * (i % 10_i64.pow(d));
                }

                if v == 0 {
                    total += i;
                    break;
                }
            }
        }
    }

    total
}
