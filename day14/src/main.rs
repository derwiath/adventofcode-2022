#![allow(dead_code)]

use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Vector2 {
    x: isize,
    y: isize,
}

impl Vector2 {
    const fn new(x: isize, y: isize) -> Vector2 {
        Vector2 { x, y }
    }

    fn add(&self, rhs: &Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
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

#[derive(PartialEq)]
enum Line {
    Horz(Vector2, Vector2),
    Vert(Vector2, Vector2),
}

impl Line {
    fn new(p1: &Vector2, p2: &Vector2) -> Line {
        if p1.x == p2.x {
            if p1.y < p2.y {
                Line::Vert(*p1, *p2)
            } else {
                Line::Vert(*p2, *p1)
            }
        } else {
            if p1.x < p2.x {
                Line::Horz(*p1, *p2)
            } else {
                Line::Horz(*p2, *p1)
            }
        }
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Line::Horz(p1, p2) => write!(f, "H {:?}, {:?}", p1, p2),
            Line::Vert(p1, p2) => write!(f, "V {:?}, {:?}", p1, p2),
        }
    }
}

fn parse_lines(s: &str) -> Result<Vec<Line>, &'static str> {
    let mut prev_point: Option<Vector2> = None;
    let mut lines: Vec<Line> = Vec::new();
    for p_str in s.split(" -> ") {
        let p = Vector2::from_str(p_str)?;
        if let Some(p1) = &prev_point {
            lines.push(Line::new(p1, &p))
        }
        prev_point = Some(p);
    }
    Ok(lines)
}

fn find_line_at(lines: &[Line], p: &Vector2) -> bool {
    lines
        .iter()
        .find(|l| match l {
            Line::Horz(p1, p2) => p.y == p1.y && p1.x <= p.x && p.x <= p2.x,
            Line::Vert(p1, p2) => p.x == p1.x && p1.y <= p.y && p.y <= p2.y,
        })
        .is_some()
}

fn solve_part1(input: &str) -> usize {
    let lines: Vec<Line> = input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| parse_lines(l))
        .map(|l| l.unwrap())
        .flatten()
        .collect();

    let max_y = lines
        .iter()
        .map(|l| match l {
            Line::Horz(p1, p2) => p1.y.max(p2.y),
            Line::Vert(p1, p2) => p1.y.max(p2.y),
        })
        .max()
        .unwrap();

    const SOURCE: Vector2 = Vector2::new(500, 0);
    const MOVES: [Vector2; 3] = [Vector2::new(0, 1), Vector2::new(-1, 1), Vector2::new(1, 1)];
    let mut sands: HashSet<Vector2> = HashSet::new();
    let mut sand = SOURCE.clone();
    while sand.y <= max_y {
        if let Some(next_sand) = MOVES.iter().find_map(|m| {
            let candidate = sand.add(m);
            if !sands.contains(&candidate) && !find_line_at(&lines[..], &candidate) {
                Some(candidate)
            } else {
                None
            }
        }) {
            sand = next_sand
        } else {
            sands.insert(sand);
            sand = SOURCE;
        }
    }

    sands.len()
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

    #[test]
    fn test1_parse_lines_1() {
        assert_eq!(
            parse_lines("498,4 -> 498,6 -> 496,6"),
            Ok(vec![
                Line::Vert(Vector2::new(498, 4), Vector2::new(498, 6)),
                Line::Horz(Vector2::new(498, 6), Vector2::new(496, 6))
            ])
        );
    }

    #[test]
    fn test1_find_line_at_1() {
        let vert_line: [Line; 1] = [Line::new(&Vector2::new(498, 4), &Vector2::new(498, 6))];
        assert_eq!(find_line_at(&vert_line, &Vector2::new(498, 4)), true);
        assert_eq!(find_line_at(&vert_line, &Vector2::new(498, 5)), true);
        assert_eq!(find_line_at(&vert_line, &Vector2::new(498, 6)), true);
        assert_eq!(find_line_at(&vert_line, &Vector2::new(497, 5)), false);
    }

    #[test]
    fn test1_find_line_at_2() {
        let horz_line: [Line; 1] = [Line::new(&Vector2::new(498, 6), &Vector2::new(496, 6))];
        assert_eq!(find_line_at(&horz_line, &Vector2::new(498, 6)), true);
        assert_eq!(find_line_at(&horz_line, &Vector2::new(497, 6)), true);
        assert_eq!(find_line_at(&horz_line, &Vector2::new(496, 6)), true);
        assert_eq!(find_line_at(&horz_line, &Vector2::new(497, 5)), false);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
