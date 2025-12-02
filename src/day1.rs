use std::fs;

pub fn run() -> i64 {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();

    let steps: Vec<i64> = input
        .split("\n")
        .filter_map(|seg| {
            if !seg.is_empty() {
                let dir = if &seg[0..1] == "L" { -1 } else { 1 };

                Some(seg[1..].parse::<i64>().unwrap() * dir)
            } else {
                None
            }
        })
        .collect();

    let mut cur = 50;
    let mut passes = 0;

    for step in steps {
        let prev = cur;
        let prevc = passes;

        passes += step.abs() / 100;

        cur += step % 100;

        if (cur <= 0 || cur > 99) && prev != 0 {
            passes += 1;
        }

        cur %= 100;

        if cur < 0 {
            cur += 100;
        }

        println!(
            "{step} from {prev} = {cur}, passed {} times",
            passes - prevc
        );
    }

    passes
}
