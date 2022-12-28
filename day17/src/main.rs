#![allow(dead_code)]

use std::env;
use std::fs;

enum Push {
    Left,
    Right,
}

impl Push {
    fn from_str(s: &str) -> Vec<Push> {
        s.chars()
            .map(|c| {
                if c == '<' {
                    Push::Left
                } else if c == '>' {
                    Push::Right
                } else {
                    panic!("Unknown push {}", c);
                }
            })
            .collect()
    }
}

struct Rock {
    x: u8,
    y: usize,
    rows: u16,
}

impl Rock {
    fn new(x: u8, y: usize, rows: u16) -> Rock {
        Rock { x, y, rows }
    }

    fn row(&self, i: usize) -> u8 {
        assert!(i < 4);
        (self.rows >> (i * 4) & 0b1111) as u8
    }

    fn shifted_row(&self, i: usize) -> Option<u8> {
        const UNSET_SHIFT_MASK: [u8; 3] = [
            0b00010000, // 5
            0b00110000, // 6
            0b01110000, // 7
        ];

        assert!(self.x < 8);
        let r = self.row(i) << 4;
        if self.x < 5 {
            Some(r >> self.x)
        } else if r & UNSET_SHIFT_MASK[(self.x - 5) as usize] == 0b00000000 {
            Some(r >> self.x)
        } else {
            None
        }
    }
}

fn solve_part1(input: &str) -> usize {
    let pushes = Push::from_str(input);
    pushes.len()
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
mod tests_day17 {
    use super::*;

    const EXAMPLE1: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 3068);
    }

    #[test]
    fn test1_rock_row_1() {
        assert_eq!(Rock::new(0, 0, 0b1111).row(0), 0b1111);
        assert_eq!(Rock::new(0, 0, 0b1111).row(1), 0b0000);
        assert_eq!(Rock::new(0, 0, 0b1111).row(2), 0b0000);
        assert_eq!(Rock::new(0, 0, 0b1111).row(3), 0b0000);
    }

    #[test]
    fn test1_rock_shifted_row_1() {
        assert_eq!(Rock::new(0, 0, 0b1111).shifted_row(0), Some(0b11110000));
        assert_eq!(Rock::new(1, 0, 0b1111).shifted_row(0), Some(0b01111000));
        assert_eq!(Rock::new(2, 0, 0b1111).shifted_row(0), Some(0b00111100));
        assert_eq!(Rock::new(3, 0, 0b1111).shifted_row(0), Some(0b00011110));
        assert_eq!(Rock::new(4, 0, 0b1111).shifted_row(0), Some(0b00001111));
        assert_eq!(Rock::new(5, 0, 0b1110).shifted_row(0), Some(0b00000111));
        assert_eq!(Rock::new(6, 0, 0b1100).shifted_row(0), Some(0b00000011));
        assert_eq!(Rock::new(7, 0, 0b1000).shifted_row(0), Some(0b00000001));
    }

    #[test]
    fn test1_rock_shifted_row_2() {
        assert_eq!(Rock::new(5, 0, 0b0001).shifted_row(0), None);
        assert_eq!(Rock::new(6, 0, 0b0010).shifted_row(0), None);
        assert_eq!(Rock::new(7, 0, 0b0100).shifted_row(0), None);
    }

    const EXAMPLE2: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2), 0);
    }
}
