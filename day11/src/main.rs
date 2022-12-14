#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::str::FromStr;

fn starting_items_from_str(s: &str) -> Result<Vec<usize>, String> {
    // Starting items: 79, 98
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"Starting items: ([0-9, ]*)").unwrap();
    }
    if let Some(captures) = RE.captures(s) {
        assert_eq!(captures.len(), 2);
        let items_str = captures.get(1).unwrap().as_str();
        let items: Vec<usize> = items_str
            .split(',')
            .map(|item_str| item_str.trim_start().trim_end())
            .map(|item_str| item_str.parse::<usize>().unwrap())
            .collect();
        Ok(items)
    } else {
        Err(format!("Failed to match test action regexp for '{}'", s))
    }
}

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

#[derive(Debug, PartialEq)]
struct Test {
    denom: usize,
    throw_to_true: usize,
    throw_to_false: usize,
}

impl FromStr for Test {
    type Err = String;
    fn from_str(s: &str) -> Result<Test, Self::Err> {
        assert_eq!(s.lines().count(), 3);
        let mut lines = s.lines();
        let line0: &str = lines.next().unwrap();
        let line1: &str = lines.next().unwrap();
        let line2: &str = lines.next().unwrap();
        let denom = Test::denom_from_str(line0)?;
        let throw1 = Test::action_from_str(line1)?;
        let throw2 = Test::action_from_str(line2)?;
        let throws = if throw1.0 == true {
            (throw1.1, throw2.1)
        } else {
            (throw2.1, throw1.1)
        };
        Ok(Test::new(denom, throws.0, throws.1))
    }
}

impl Test {
    fn new(denom: usize, throw_to_true: usize, throw_to_false: usize) -> Test {
        Test {
            denom,
            throw_to_true,
            throw_to_false,
        }
    }

    fn denom_from_str(s: &str) -> Result<usize, String> {
        //  Test: divisible by 19
        lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"Test: divisible by (\d*)").unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            assert_eq!(captures.len(), 2);
            let denom = captures.get(1).unwrap().as_str();
            if let Ok(denom_number) = denom.parse::<usize>() {
                Ok(denom_number)
            } else {
                Err("Failed to parse denom number".to_string())
            }
        } else {
            Err(format!("Failed to match denom regexp for '{}'", s))
        }
    }

    fn action_from_str(s: &str) -> Result<(bool, usize), String> {
        //    If true: throw to monkey 2
        //    If false: throw to monkey 0
        lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new(r"^.*If ([a-z]*): throw to monkey ([0-9]*)").unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            assert_eq!(captures.len(), 3);
            let cond_str = captures.get(1).unwrap().as_str();
            let cond = match cond_str.parse::<bool>() {
                Ok(cond) => cond,
                Err(_) => return Err("Failed to parse bool condition".to_string()),
            };
            let monkey_str = captures.get(2).unwrap().as_str();
            let monkey = match monkey_str.parse::<usize>() {
                Ok(monkey) => monkey,
                Err(_) => return Err("Failed to parse monkey number".to_string()),
            };
            Ok((cond, monkey))
        } else {
            Err(format!("Failed to match test action regexp for '{}'", s))
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
    fn test1_items_1() {
        assert_eq!(
            starting_items_from_str("Starting items: 79, 60, 97"),
            Ok(vec![79, 60, 97])
        );
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

    #[test]
    fn test1_test_1() {
        let s: &str = "Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0";
        assert_eq!(Test::from_str(s), Ok(Test::new(19, 2, 0)));
    }

    #[test]
    fn test1_test_2() {
        let s: &str = "If true: throw to monkey 2";
        assert_eq!(Test::action_from_str(s), Ok((true, 2)));
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
