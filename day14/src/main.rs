#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Vector2 {
    fn new(x: isize, y: isize) -> Vector2 {
        Vector2 { x, y }
    }
}

impl FromStr for Vector2 {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Vector2, &'static str> {
        let mut it = input.split(",");
        let x = if let Some(x_str) = it.next() {
            if let Ok(x) = x_str.parse::<usize>() {
                x
            } else {
                return Err("Failed to parse x");
            }
        } else {
            return Err("No x in str");
        };
        let y = if let Some(x_str) = it.next() {
            if let Ok(y) = x_str.parse::<usize>() {
                y
            } else {
                return Err("Failed to parse y");
            }
        } else {
            return Err("No y in str");
        };
        Ok(Vector2::new(x as isize, y as isize))
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
mod tests_day14 {
    use super::*;

    const EXAMPLE1: &str = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 24);
    }

    #[test]
    fn test1_vector2_from_str_1() {
        assert_eq!(Vector2::from_str("498,4"), Ok(Vector2::new(498, 4)));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
