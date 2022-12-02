#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Pick {
    Rock,
    Paper,
    Scizzors,
}

impl Pick {
    fn new(s: &str) -> Pick {
        match s {
            "A" | "X" => Pick::Rock,
            "B" | "Y" => Pick::Paper,
            "C" | "Z" => Pick::Scizzors,
            _ => panic!("not valid pick"),
        }
    }

    fn play(&self, other: &Pick) -> usize {
        match *self {
            Pick::Rock => match *other {
                Pick::Rock => 1 + 3,
                Pick::Paper => 1,
                Pick::Scizzors => 1 + 6,
            },
            Pick::Paper => match *other {
                Pick::Rock => 2 + 6,
                Pick::Paper => 2 + 3,
                Pick::Scizzors => 2 + 0,
            },
            Pick::Scizzors => match *other {
                Pick::Rock => 3 + 0,
                Pick::Paper => 3 + 6,
                Pick::Scizzors => 3 + 3,
            },
        }
    }
}

fn solve_part1(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"([A-C]) ([X-Z])").unwrap();
    }
    let mut sum = 0;
    for line in input.lines() {
        if let Some(captures) = RE.captures(line) {
            assert!(captures.len() == 3);
            if captures.len() == 3 {
                let pick1_str = captures.get(1).unwrap().as_str();
                let pick2_str = captures.get(2).unwrap().as_str();
                let pick1 = Pick::new(pick1_str);
                let pick2 = Pick::new(pick2_str);
                let score = pick2.play(&pick1);
                println!("{:?} {:?} -> {}", &pick1, &pick2, score);
                assert!(score > 0);
                sum += score
            }
        } else if line.trim().len() > 0 {
            panic!("Failed to parse line: <{}>", line);
        }
    }
    sum
}

fn solve_part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    const PACKAGE_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    let filename = args
        .get(1)
        .expect(format!("Usage: {} input-filename", PACKAGE_NAME.unwrap()).as_str());

    println!("Reading input from {}", filename);
    let input = fs::read_to_string(filename).expect("Failed to read file");

    let answer1 = solve_part1(&input);
    println!("Answer 1: {}", answer1);

    let answer2 = solve_part2(&input);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests_day2 {
    use super::*;

    const EXAMPLE1: &str = "
A Y
B X
C Z";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 15);
    }

    const EXAMPLE2: &str = "15";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
