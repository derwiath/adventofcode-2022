use std::collections::HashSet;
use std::env;
use std::fs;

struct Forrest {
    row_major: Vec<u8>,
    column_major: Vec<u8>,
    width: usize,
    height: usize,
}

impl Forrest {
    fn from_str(input: &str) -> Forrest {
        let mut width = 0;
        let row_major: Vec<u8> = input
            .lines()
            .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
            .map(|l| {
                let mut row: Vec<u8> = Vec::new();
                for c in l.chars() {
                    assert!(c.is_digit(10));
                    let digit = c.to_digit(10).unwrap();
                    row.push(digit as u8);
                }
                width += 1;
                row
            })
            .flatten()
            .collect();
        let height = row_major.len() / width;
        let mut column_major: Vec<u8> = Vec::with_capacity(row_major.len());

        for x in 0..width {
            for y in 0..height {
                let tree_index = width * y + x;
                column_major.push(row_major[tree_index]);
            }
        }

        return Forrest {
            row_major,
            column_major,
            width,
            height,
        };
    }

    fn border_visible(&self) -> usize {
        self.width * 2 + (self.height - 2) * 2
    }

    fn get_row(&self, y: usize) -> &[u8] {
        let row_start_index = y * self.width;
        &self.row_major[row_start_index..row_start_index + self.width]
    }

    fn get_column(&self, x: usize) -> &[u8] {
        let column_start_index = x * self.height;
        &self.column_major[column_start_index..column_start_index + self.height]
    }
}

fn solve_part1(input: &str) -> usize {
    let forrest = Forrest::from_str(input);
    let mut visible: HashSet<usize> = HashSet::new();

    for y in 1..forrest.height - 1 {
        let row_start_index = y * forrest.width;
        let row = &forrest.get_row(y);
        let mut left_max = row[0];
        let mut left_max_index = 0;
        for x in 1..forrest.width - 1 {
            if row[x] > left_max {
                visible.insert(row_start_index + x);
                left_max_index = x;
                left_max = row[x];
            }
        }

        let mut right_max = row[forrest.width - 1];
        for x in (left_max_index + 1..forrest.width - 1).rev() {
            if row[x] > right_max {
                visible.insert(row_start_index + x);
                right_max = row[x];
            }
        }
    }

    for x in 1..forrest.width - 1 {
        let column = forrest.get_column(x);
        let mut top_max = column[0];
        let mut top_max_y = 0;
        for y in 1..forrest.height - 1 {
            let height = column[y];
            if height > top_max {
                let tree_index = forrest.width * y + x;
                visible.insert(tree_index);
                top_max_y = y;
                top_max = height;
            }
        }

        let mut bottom_max = column[forrest.height - 1];
        for y in (top_max_y + 1..forrest.height - 1).rev() {
            let height = column[y];
            if height > bottom_max {
                let tree_index = forrest.width * y + x;
                visible.insert(tree_index);
                bottom_max = height;
            }
        }
    }

    forrest.border_visible() + visible.len()
}

fn score(forrest: &Forrest, x: usize, y: usize) -> usize {
    let row = forrest.get_row(y);
    let column = forrest.get_column(x);
    let tree_height = row[x];

    let right_less_view = row[x + 1..]
        .iter()
        .take_while(|h| h < &&tree_height)
        .count();
    let right_view = right_less_view
        + if x + right_less_view + 1 < row.len() && row[x + right_less_view + 1] == tree_height {
            1
        } else {
            0
        };

    let left_less_view = row[0..x]
        .iter()
        .rev()
        .take_while(|h| h < &&tree_height)
        .count();

    // 30373
    // 25512
    // 65332
    // 33549
    // 35390
    let left_view = left_less_view
        + if x > left_less_view && row[x - left_less_view - 1] == tree_height {
            1
        } else {
            0
        };

    let up_less_view = column[0..y]
        .iter()
        .rev()
        .take_while(|h| h < &&tree_height)
        .count();
    let up_view = up_less_view
        + if y > up_less_view && column[y - up_less_view - 1] == tree_height {
            1
        } else {
            0
        };
    let down_less_view = column[y + 1..]
        .iter()
        .take_while(|h| h < &&tree_height)
        .count();
    let down_view = down_less_view
        + if y + down_less_view + 1 < column.len() && column[y + down_less_view + 1] == tree_height
        {
            1
        } else {
            0
        };

    let s = [left_view, right_view, up_view, down_view]
        .into_iter()
        .map(|s| s.max(1))
        .fold(1, |acc, s| acc * s);
    println!("({}, {}) = {}", x, y, s);
    /*
    println!(" row {:?}", row);
    println!(" column {:?}", column);
    println!(" left  {} left_less  {}", left_view, left_less_view);
    println!(" right {} right_less {}", right_view, right_less_view);
    println!(" up    {} up_less    {}", up_view, up_less_view);
    println!(" down  {} down_less  {}", down_view, down_less_view);
    */

    s
}

fn solve_part2(input: &str) -> usize {
    let forrest = Forrest::from_str(input);
    let mut m = 0;

    for x in 0..forrest.width {
        for y in 0..forrest.height {
            let s = score(&forrest, x, y);
            m = s.max(m);
        }
    }
    m
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
mod tests_day8 {
    use super::*;

    const EXAMPLE1: &str = "
30373
25512
65332
33549
35390";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 21);
    }

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE1), 8);
    }

    #[test]
    fn test2_2() {
        let forrest = Forrest::from_str(EXAMPLE1);
        assert_eq!(score(&forrest, 2, 1), 4);
    }

    #[test]
    fn test2_3() {
        let forrest = Forrest::from_str(EXAMPLE1);
        assert_eq!(score(&forrest, 2, 3), 4);
    }
    #[test]
    fn test2_4() {
        let forrest = Forrest::from_str(EXAMPLE1);
        assert_eq!(score(&forrest, 0, 0), 2);
    }
    #[test]
    fn test2_5() {
        let forrest = Forrest::from_str(EXAMPLE1);
        assert_eq!(score(&forrest, 0, 2), 2);
    }
}
