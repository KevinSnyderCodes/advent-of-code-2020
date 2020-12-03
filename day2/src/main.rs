use std::env;
use std::fs;

use regex::Regex;

struct Line {
    n1: i32,
    n2: i32,
    c: char,
    value: String,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();

        let captures = re.captures(s).expect("Malformatted string");

        return Line {
            n1: captures[1].parse().expect("First number is incorrect"),
            n2: captures[2].parse().expect("Second number is incorrect"),
            c: captures[3].chars().nth(0).expect("Character is incorrect"),
            value: String::from(&captures[4]),
        };
    }
}

impl Line {
    fn is_valid_part1(&self) -> bool {
        let mut count = 0;

        for c in self.value.chars() {
            if c == self.c {
                count = count + 1
            }
        }

        return self.n1 <= count && count <= self.n2;
    }

    fn is_valid_part2(&self) -> bool {
        let c1 = self
            .value
            .chars()
            .nth((self.n1 - 1) as usize)
            .expect("Error with first position");
        let c2 = self
            .value
            .chars()
            .nth((self.n2 - 1) as usize)
            .expect("Error with first position");

        let mut count = 0;

        if c1 == self.c {
            count = count + 1
        }
        if c2 == self.c {
            count = count + 1
        }

        return count == 1;
    }
}

// O(n)
fn part1(lines: &Vec<Line>) -> i32 {
    let valid: Vec<&Line> = lines.iter().filter(|line| line.is_valid_part1()).collect();

    return valid.len() as i32;
}

// O(n)
fn part2(lines: &Vec<Line>) -> i32 {
    let valid: Vec<&Line> = lines.iter().filter(|line| line.is_valid_part2()).collect();

    return valid.len() as i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<Line> = contents
        .trim()
        .split("\n")
        .map(|line| Line::from(line))
        .collect();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
