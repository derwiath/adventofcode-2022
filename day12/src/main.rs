#![allow(dead_code)]

use std::env;
use std::fmt;
use std::fs;
use std::str::FromStr;

struct Vector2 {
    x: usize,
    y: usize,
}

impl Vector2 {
    fn new(x: usize, y: usize) -> Vector2 {
        Vector2 { x, y }
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Map {
    row_major: Vec<u8>,
    size: Vector2,
    start: Vector2,
    end: Vector2,
}

impl Map {
    fn new(row_major: Vec<u8>, size: Vector2, start: Vector2, end: Vector2) -> Map {
        Map {
            row_major,
            size,
            start,
            end,
        }
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(input: &str) -> Result<Map, ()> {
        let mut width = 0;
        let mut start: Option<Vector2> = None;
        let mut end: Option<Vector2> = None;
        let row_major: Vec<u8> = input
            .lines()
            .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
            .enumerate()
            .map(|(y, l)| {
                let row: Vec<u8> = l
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let height_char = match c {
                            'a'..='z' => c,
                            'S' => {
                                start = Some(Vector2::new(x, y));
                                'a'
                            }
                            'E' => {
                                end = Some(Vector2::new(x, y));
                                'z'
                            }
                            _ => panic!("unknown char {}", c),
                        };
                        height_char as u8 - 'a' as u8
                    })
                    .collect();
                width = row.len();
                row
            })
            .flatten()
            .collect();
        let height = row_major.len() / width;
        let size = Vector2::new(width, height);
        Ok(Map::new(
            row_major,
            size,
            start.expect("No start (S) found on map"),
            end.expect("No end (E) found on Map"),
        ))
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.y {
            self.row_major[y * self.size.x..(y + 1) * self.size.x]
                .iter()
                .for_each(|h| {
                    let c = (h + 'a' as u8) as char;
                    write!(f, "{}", c).expect("Failed to write");
                });
            writeln!(f, "")?;
        }
        writeln!(f, "size: {}", self.size)?;
        writeln!(f, "start: {}", self.start)?;
        writeln!(f, "end: {}", self.end)?;
        Ok(())
    }
}

fn solve_part1(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();

    println!("{}", map);

    map.size.x * map.size.y
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
mod tests_day12 {
    use super::*;

    const EXAMPLE1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 31);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
