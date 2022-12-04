#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Range {
        Range { start, end }
    }

    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

#[derive(Debug)]
struct Pair {
    r1: Range,
    r2: Range,
}

impl Pair {
    fn new(r1: Range, r2: Range) -> Pair {
        Pair { r1, r2 }
    }

    fn contains(&self) -> bool {
        self.r1.contains(&self.r2) || self.r2.contains(&self.r1)
    }

    fn overlaps(&self) -> bool {
        self.r1.overlaps(&self.r2) || self.r2.overlaps(&self.r1)
    }
}

impl FromStr for Pair {
    type Err = ParsePairError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn solve_part1(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    }
    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            assert_eq!(captures.len(), 5);
            let digits = [
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            ];
            Pair::new(
                Range::new(digits[0], digits[1]),
                Range::new(digits[2], digits[3]),
            )
        })
        .map(|p| if p.contains() { 1 } else { 0 })
        .fold(0, |acc, c| acc + c)
}

fn solve_part2(input: &str) -> usize {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    }

    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            assert_eq!(captures.len(), 5);
            let digits = [
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            ];
            Pair::new(
                Range::new(digits[0], digits[1]),
                Range::new(digits[2], digits[3]),
            )
        })
        .map(|p| if p.overlaps() { 1 } else { 0 })
        .fold(0, |acc, c| acc + c)
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
mod tests_day4 {
    use super::*;

    const EXAMPLE1: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 2);
    }

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE1), 4);
    }
}
