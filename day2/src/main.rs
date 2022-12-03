#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

#[repr(u8)]
#[derive(PartialEq, Debug, Copy, Clone)]
enum Pick {
    Rock = 1,
    Scizzors = 2,
    Paper = 3,
}

#[repr(u8)]
#[derive(PartialEq, Debug, Copy, Clone)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
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

    fn from_u8(value: u8) -> Pick {
        match value {
            1 => Pick::Rock,
            2 => Pick::Scizzors,
            3 => Pick::Paper,
            _ => panic!("not valid pick value"),
        }
    }

    fn play(&self, other: &Pick) -> Outcome {
        if *self == *other {
            Outcome::Draw
        } else if (*self as u8 + 1) % 3 == (*other as u8) {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn score(&self) -> usize {
        match *self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scizzors => 3,
        }
    }

    fn pick_for_outcome_against_self(&self, outcome: &Outcome) -> Pick {
        let pick_offset = match outcome {
            Outcome::Win => 2,
            Outcome::Lose => 1,
            Outcome::Draw => 0,
        };
        Pick::from_u8((*self as u8 + pick_offset) % 3)
    }
}

impl Outcome {
    fn new(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("{} is not valid outcome", s),
        }
    }

    fn score(&self) -> usize {
        *self as usize
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
                let outcome = pick2.play(&pick1);
                let score = pick2.score() + outcome.score();
                sum += score
            }
        } else if line.trim().len() > 0 {
            panic!("Failed to parse line: <{}>", line);
        }
    }
    sum
}

fn solve_part2(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"([A-C]) ([X-Z])").unwrap();
    }
    let mut sum = 0;
    for line in input.lines() {
        if let Some(captures) = RE.captures(line) {
            assert!(captures.len() == 3);
            if captures.len() == 3 {
                let pick1_str = captures.get(1).unwrap().as_str();
                let outcome_str = captures.get(2).unwrap().as_str();
                let pick1 = Pick::new(pick1_str);
                let outcome = Outcome::new(outcome_str);
                let pick2 = pick1.pick_for_outcome_against_self(&outcome);
                let score = pick2.score() + outcome.score();
                sum += score
            }
        } else if line.trim().len() > 0 {
            print!("line: \"{}\"", line);
            panic!("Failed to parse line");
        }
    }
    sum
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

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE1), 12);
    }
}
