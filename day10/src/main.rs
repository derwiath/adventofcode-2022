#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum Instr {
    Noop,
    Addx(isize),
}

impl Instr {
    fn from_str(s: &str) -> Instr {
        if s == "noop" {
            Instr::Noop
        } else if s.starts_with("addx ") {
            let number = s["addx ".len()..].parse::<isize>().unwrap();
            Instr::Addx(number)
        } else {
            panic!("Failed to parse instruction from: {}", s);
        }
    }
}

fn solve_part1(input: &str) -> isize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d*) ([a-z]*)").unwrap();
    }
    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            assert_eq!(captures.len(), 3);
            let count: isize = captures.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let thing = captures.get(2).unwrap().as_str();
            (count, thing)
        })
        .fold(0, |acc, (count, _)| acc + count)
}

fn solve_part2(input: &str) -> isize {
    input.len() as isize
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
mod tests_day10 {
    use super::*;

    const EXAMPLE1: &str = "
3 seals
4 quacks";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 7);
    }

    #[test]
    fn test_instr_1() {
        assert_eq!(Instr::from_str("noop"), Instr::Noop);
    }

    #[test]
    fn test_instr_2() {
        assert_eq!(Instr::from_str("addx 314"), Instr::Addx(314));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
