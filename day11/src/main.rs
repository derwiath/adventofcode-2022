#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Add(usize), // new = old + x
    Mul(usize), // new = old * x
    Sqr,        // new = old * old
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        // Operation: new = old * 19
        // Operation: new = old + 6
        // Operation: new = old * old
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"Operation: new = old ([+*]) ([\da-z]*)").unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            assert_eq!(captures.len(), 3);
            let operand = captures.get(1).unwrap().as_str();
            let x = captures.get(2).unwrap().as_str();
            if x == "old" {
                assert_eq!(operand, "*");
                Ok(Operation::Sqr)
            } else {
                match x.parse::<usize>() {
                    Ok(x_number) => {
                        if operand == "+" {
                            Ok(Operation::Add(x_number))
                        } else {
                            assert_eq!(operand, "*");
                            Ok(Operation::Mul(x_number))
                        }
                    }
                    _ => Err("Failed to parse number".to_string()),
                }
            }
        } else {
            Err(format!("Failed to match operation regexp for '{}'", s))
        }
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
mod tests_day11 {
    use super::*;

    const EXAMPLE1: &str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 10605);
    }

    #[test]
    fn test1_op_1() {
        assert_eq!(
            Operation::from_str("  Operation: new = old * 19"),
            Ok(Operation::Mul(19))
        );
    }

    #[test]
    fn test1_op_2() {
        assert_eq!(
            Operation::from_str("  Operation: new = old * old"),
            Ok(Operation::Sqr)
        );
    }

    #[test]
    fn test1_op_3() {
        assert_eq!(
            Operation::from_str("  Operation: new = old + 3"),
            Ok(Operation::Add(3))
        );
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
