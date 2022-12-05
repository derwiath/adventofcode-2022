#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(count: usize, from: usize, to: usize) -> Move {
        Move { count, from, to }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"move (\d) from (\d) to (\d)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        assert_eq!(captures.len(), 4);
        let count: usize = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from: usize = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to: usize = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        Ok(Self { count, from, to })
    }
}

fn solve_part1(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d*) ([a-z]*)").unwrap();
    }
    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            assert_eq!(captures.len(), 3);
            let count: usize = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let thing = captures.get(2).unwrap().as_str();
            (count, thing)
        })
        .fold(0, |acc, (count, _)| acc + count)
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
mod tests_day5 {
    use super::*;

    const EXAMPLE1: &str = "
3 seals
4 quacks";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 7);
    }

    #[test]
    fn test1_move_from_str() {
        assert_eq!(Move::from_str("move 2 from 4 to 6"), Ok(Move::new(2, 4, 6)));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
