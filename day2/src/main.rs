#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Pick {
    Rock,
    Scizzors,
    Paper,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
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

    fn play(&self, other: &Pick) -> Outcome {
        match *self {
            Pick::Rock => match *other {
                Pick::Rock => Outcome::Draw,
                Pick::Paper => Outcome::Lose,
                Pick::Scizzors => Outcome::Win,
            },
            Pick::Paper => match *other {
                Pick::Rock => Outcome::Win,
                Pick::Paper => Outcome::Draw,
                Pick::Scizzors => Outcome::Lose,
            },
            Pick::Scizzors => match *other {
                Pick::Rock => Outcome::Lose,
                Pick::Paper => Outcome::Win,
                Pick::Scizzors => Outcome::Draw,
            },
        }
    }

    fn score(&self) -> usize {
        match *self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scizzors => 3,
        }
    }

    fn pick_wins_over_self(&self) -> Pick {
        match *self {
            Pick::Rock => Pick::Paper,
            Pick::Paper => Pick::Scizzors,
            Pick::Scizzors => Pick::Rock,
        }
    }

    fn pick_looses_to_self(&self) -> Pick {
        match *self {
            Pick::Rock => Pick::Scizzors,
            Pick::Paper => Pick::Rock,
            Pick::Scizzors => Pick::Paper,
        }
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
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
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
                let outcome = pick2.play(&pick1);
                let score = pick2.score() + outcome.score();
                println!(
                    "{:?} {:?} -> {} ({} + {})",
                    &pick1,
                    &pick2,
                    score,
                    pick2.score(),
                    outcome.score()
                );
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
                let outcome = Outcome::new(pick2_str);
                let pick2 = match outcome {
                    Outcome::Win => pick1.pick_wins_over_self(),
                    Outcome::Lose => pick1.pick_looses_to_self(),
                    Outcome::Draw => pick1.clone(),
                };

                let score = pick2.score() + outcome.score();
                println!(
                    "{:?} {:?} -> {} ({} + {})",
                    &pick1,
                    &pick2,
                    score,
                    pick2.score(),
                    outcome.score()
                );
                assert!(score > 0);
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
