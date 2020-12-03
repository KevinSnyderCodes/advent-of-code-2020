use std::env;
use std::fs;

const WANT: i32 = 2020;

// O(n * log(n))
fn part1_sort(mut numbers: Vec<i32>) -> i32 {
    numbers.sort();

    let mut i = 0;
    let mut j = numbers.len() - 1;

    loop {
        let sum = numbers[i] + numbers[j];
        if sum < WANT {
            i = i + 1;
        } else if sum > WANT {
            j = j - 1;
        } else {
            break;
        }
    }

    let result = numbers[i] * numbers[j];

    return result;
}

// O(n^2)
fn part1_naive(numbers: &Vec<i32>) -> i32 {
    for a in numbers {
        for b in numbers {
            if a + b == WANT {
                return a * b;
            }
        }
    }

    return 0;
}

// O(n^3)
fn part2_naive(numbers: &Vec<i32>) -> i32 {
    for a in numbers {
        for b in numbers {
            for c in numbers {
                if a + b + c == WANT {
                    return a * b * c;
                }
            }
        }
    }

    let result = 0;

    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents
        .trim()
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();

    println!("{}", part1_sort(numbers.clone()));
    println!("{}", part1_naive(&numbers));
    println!("{}", part2_naive(&numbers));
}
