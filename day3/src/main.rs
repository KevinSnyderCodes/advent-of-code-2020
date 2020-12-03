use std::env;
use std::fs;

// O(n)
fn part1(grid: &Vec<Vec<bool>>) -> i64 {
    return num_trees(grid, &(3, 1));
}

// O(n)
fn part2(grid: &Vec<Vec<bool>>) -> i64 {
    return Vec::from([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
        .iter()
        .map(|incr| num_trees(grid, incr))
        .fold(1, |acc, count| acc * count);
}

// O(n)
fn num_trees(grid: &Vec<Vec<bool>>, incr: &(usize, usize)) -> i64 {
    let len_y = grid.len();
    if len_y == 0 {
        return 0;
    }

    let len_x = grid[0].len();
    if len_x == 0 {
        return 0;
    }

    let mut y = 0;
    let mut x = 0;

    let mut count = 0;

    while y < len_y {
        if grid[y][x] {
            count = count + 1;
        }

        y = y + incr.1;
        x = (x + incr.0) % len_x;
    }

    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let grid: Vec<Vec<bool>> = contents
        .trim()
        .split("\n")
        .map(|line| {
            String::from(line.trim())
                .chars()
                .map(|c| c == '#')
                .collect()
        })
        .collect();

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
