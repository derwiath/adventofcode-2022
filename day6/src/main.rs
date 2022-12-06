use std::env;
use std::fs;

fn find_marker_pos(input: &str, marker_len: usize) -> usize {
    if input.len() < marker_len {
        return 0;
    }
    let process_count = input.len() - marker_len;
    let mut i = 0;
    while i < process_count {
        let candidate = &input[i..i + marker_len];

        let mut unique_count = 0;
        for (j, needle) in candidate.chars().enumerate() {
            if candidate[j + 1..].chars().find(|c| c == &needle).is_none() {
                unique_count += 1;
            } else {
                break;
            }
        }
        if unique_count == marker_len {
            return i + unique_count;
        }
        i += unique_count + 1;
    }
    return 0;
}

fn solve_part1(input: &str) -> usize {
    find_marker_pos(input, 4)
}

fn solve_part2(input: &str) -> usize {
    find_marker_pos(input, 14)
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
mod tests_day6 {
    use super::*;

    const EXAMPLE1_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE1_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE1_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE1_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE1_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1_1), 7);
        assert_eq!(solve_part1(EXAMPLE1_2), 5);
        assert_eq!(solve_part1(EXAMPLE1_3), 6);
        assert_eq!(solve_part1(EXAMPLE1_4), 10);
        assert_eq!(solve_part1(EXAMPLE1_5), 11);
    }

    const EXAMPLE2_1: &str = "";

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE2_1), 0);
    }
}
