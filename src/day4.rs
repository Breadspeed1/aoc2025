pub fn run() -> i64 {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();

    let mut grid = input
        .lines()
        .into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let permutations = [
        [0, 1],
        [1, 0],
        [1, 1],
        [-1, 1],
        [1, -1],
        [-1, 0],
        [0, -1],
        [-1, -1],
    ];

    let mut rolls = 0;

    loop {
        let mut to_remove: Vec<[usize; 2]> = vec![];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut total = 0;

                if grid[i][j] != '@' {
                    continue;
                }

                for [x, y] in permutations {
                    let newx = i as i32 + x;
                    let newy = j as i32 + y;

                    if newx >= 0
                        && newy >= 0
                        && newx < grid.len() as i32
                        && newy < grid[0].len() as i32
                    {
                        if grid[newx as usize][newy as usize] == '@' {
                            total += 1;
                        }
                    }
                }

                if total < 4 {
                    to_remove.push([i, j]);
                    rolls += 1;
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for [i, j] in to_remove {
            grid[i][j] = 'x';
        }
    }

    rolls
}
