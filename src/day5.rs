use std::collections::HashSet;

pub fn run() -> i64 {
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();

    let split: Vec<String> = input
        .trim()
        .split("\n\n")
        .map(ToString::to_string)
        .collect();

    let mut ranges: Vec<(i64, i64)> = split[0]
        .split("\n")
        .map(|v| {
            let s: Vec<&str> = v.split("-").collect();

            (s[0].parse::<i64>().unwrap(), s[1].parse::<i64>().unwrap())
        })
        .collect();

    let ingredients: Vec<i64> = split[1]
        .split("\n")
        .map(|v| {
            let i_v: i64 = v.parse().unwrap();

            i_v
        })
        .collect();

    ranges.sort_by(|r1, r2| r1.0.cmp(&r2.0));

    let mut v = 0;

    let (mut ps, mut pe) = ranges[0];

    for (s, e) in ranges.iter().skip(1) {
        if (pe + 1) >= *s && *e > pe {
            pe = *e;
        } else if (pe + 1) < *e {
            v += (pe - ps) + 1;
            ps = *s;
            pe = *e;
        }
    }

    v += (pe - ps) + 1;

    v
}
