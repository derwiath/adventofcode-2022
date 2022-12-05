#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Row {
    row: Vec<Option<char>>,
}

#[allow(dead_code)]
impl Row {
    fn new(row: Vec<Option<char>>) -> Row {
        Row { row }
    }

    fn len(&self) -> usize {
        self.row.len()
    }

    fn get(&self, i: usize) -> Option<char> {
        if i < self.row.len() {
            self.row[i]
        } else {
            None
        }
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CRATE_RE: regex::Regex = regex::Regex::new(r"([A-Z])").unwrap();
            static ref NONE_RE: regex::Regex = regex::Regex::new(r"(   )").unwrap();
        }
        let mut row: Vec<Option<char>> = vec![];
        let mut i = 0;
        while i + 3 <= s.len() {
            let substr = &s[i..i + 3];
            if let Some(cap) = CRATE_RE.captures(substr) {
                assert_eq!(cap.len(), 2);
                row.push(cap.get(1).unwrap().as_str().chars().next());
            } else if let Some(_) = NONE_RE.captures(substr) {
                row.push(None);
            } else {
                return Err(());
            }
            i += 4;
        }
        Ok(Self { row })
    }
}

#[derive(Debug, PartialEq)]
struct Stacks {
    stacks: Vec<String>,
}

impl Stacks {
    fn new(stacks: Vec<String>) -> Stacks {
        Stacks { stacks }
    }

    fn from_rows(rows: &Vec<Row>) -> Stacks {
        assert!(rows.len() > 0);
        let stack_count = rows[0].len();
        let mut stacks: Vec<String> = Vec::with_capacity(stack_count);

        for row in rows {
            assert_eq!(row.len(), stack_count);
        }

        for i in 0..stack_count {
            let mut s = String::with_capacity(rows.len());
            for row in rows {
                if let Some(c) = row.get(i) {
                    s.push(c);
                }
            }
            stacks.push(s);
        }

        Stacks { stacks }
    }
}

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

fn solve_part1(input: &str) -> String {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"(\d*) ([a-z]*)").unwrap();
    }
    let rows: Vec<Row> = input.lines().map_while(|l| Row::from_str(l).ok()).collect();
    let row_count = rows.len();

    let moves: Vec<Move> = input
        .lines()
        .skip(row_count + 2)
        .map(|l| Move::from_str(l).unwrap())
        .collect();

    let stacks = Stacks::from_rows(&rows);
    stacks
        .stacks
        .iter()
        .map(|s| s.chars().next().unwrap_or(' '))
        .collect::<String>()
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
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), "CMZ".to_string());
    }

    #[test]
    fn test1_move_from_str() {
        assert_eq!(Move::from_str("move 2 from 4 to 6"), Ok(Move::new(2, 4, 6)));
    }

    #[test]
    fn test1_row_1() {
        assert_eq!(
            Row::from_str("    [D]    "),
            Ok(Row::new(vec![None, Some('D'), None]))
        );
    }

    #[test]
    fn test1_row_2() {
        assert_eq!(Row::from_str(" 1   2   3 "), Err(()));
    }

    #[test]
    fn test1_stacks_1() {
        let rows: Vec<Row> = EXAMPLE1
            .lines()
            .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
            .map_while(|l| Row::from_str(&l).ok())
            .collect();
        assert_eq!(
            Stacks::from_rows(&rows),
            Stacks::new(vec!["NZ".to_string(), "DCM".to_string(), "P".to_string()])
        );
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
