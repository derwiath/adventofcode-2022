#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;

fn sign(a: isize) -> isize {
    if a > 0 {
        1
    } else if a == 0 {
        0
    } else {
        -1
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Vector2 {
    fn new(x: isize, y: isize) -> Vector2 {
        Vector2 { x, y }
    }

    fn from_move(dir: &str, count: isize) -> Vector2 {
        match dir {
            "U" => Vector2::new(0, 1 * count),
            "D" => Vector2::new(0, -1 * count),
            "L" => Vector2::new(-1 * count, 0),
            "R" => Vector2::new(1 * count, 0),
            _ => panic!("{} is not a valid dir", dir),
        }
    }

    // self + other
    fn add(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    // self - other
    fn diff(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }

    fn sign(&self) -> Vector2 {
        Vector2::new(sign(self.x), sign(self.y))
    }

    fn abs(&self) -> Vector2 {
        Vector2::new(self.x.abs(), self.y.abs())
    }

    fn min_element(&self) -> isize {
        self.x.min(self.y)
    }

    fn max_element(&self) -> isize {
        self.x.max(self.y)
    }

    fn manhattan_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    fn follow(&self, head: &Vector2) -> Vector2 {
        let diff = head.diff(&self);
        let diff_abs = diff.abs();
        let diff_sign = diff.sign();
        if diff_abs.max_element() >= 2 && diff_abs.min_element() == 0 {
            self.add(&diff_sign)
        } else if (head.x != self.x || head.y != self.y) && diff_abs.manhattan_distance() > 2 {
            self.add(&diff_sign)
        } else {
            self.clone()
        }
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn read_moves(input: &str) -> Vec<(Vector2, &str)> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"([UDRL]) (\d*)").unwrap();
    }
    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| {
            let captures = RE.captures(l).unwrap();
            assert_eq!(captures.len(), 3);
            let dir = captures.get(1).unwrap().as_str();
            let count: isize = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
            (Vector2::from_move(dir, count), dir)
        })
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let moves = read_moves(input);
    let mut head = Vector2::new(0, 0);
    let mut tail = Vector2::new(0, 0);
    let mut visited: HashSet<Vector2> = HashSet::new();
    visited.insert(tail.clone());

    for (m, _) in moves {
        let m_step = m.sign();

        for _ in 0..m.abs().max_element() {
            head = head.add(&m_step);
            tail = tail.follow(&head);
            visited.insert(tail.clone());
        }
    }
    visited.len()
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
mod tests_day9 {
    use super::*;

    const EXAMPLE1: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 13);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
