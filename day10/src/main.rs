use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum Instr {
    Noop,
    Addx(isize),
}

impl Instr {
    fn from_str(s: &str) -> Instr {
        if s == "noop" {
            Instr::Noop
        } else if s.starts_with("addx ") {
            let number = s["addx ".len()..].parse::<isize>().unwrap();
            Instr::Addx(number)
        } else {
            panic!("Failed to parse instruction from: {}", s);
        }
    }
}

fn read_instructions(input: &str) -> Vec<Instr> {
    input
        .lines()
        .filter_map(|l| if l.len() > 0 { Some(l) } else { None })
        .map(|l| Instr::from_str(l))
        .collect()
}

fn sample_signal_strength(cycle: isize, x: isize) -> Option<isize> {
    if cycle >= 20 && ((cycle - 20) % 40 == 0) {
        println!("cycle {}, x {}, strength {}", cycle, x, cycle * x);
        Some(cycle * x)
    } else {
        None
    }
}

fn solve_part1(input: &str) -> isize {
    let instructions = read_instructions(input);

    let mut x: isize = 1;
    let mut cycle: isize = 0;
    let mut sum: isize = 0;
    for (i, instr) in instructions.iter().enumerate() {
        match instr {
            Instr::Noop => {
                cycle += 1;
                sum += match sample_signal_strength(cycle, x) {
                    Some(s) => {
                        println!("{}: noop", i);
                        s
                    }
                    None => 0,
                };
            }
            Instr::Addx(y) => {
                cycle += 1;
                sum += match sample_signal_strength(cycle, x) {
                    Some(s) => {
                        println!("{}: addx({}) #1", i, y);
                        s
                    }
                    None => 0,
                };
                cycle += 1;
                sum += match sample_signal_strength(cycle, x) {
                    Some(s) => {
                        println!("{}: addx({}) #2", i, y);
                        s
                    }
                    None => 0,
                };
                x += y;
            }
        }
    }
    sum
}

fn print_screen(label: &str, s: &str) {
    println!("{}", label);
    for row in 0..6 {
        println!("{}", &s[row * 40..(row + 1) * 40]);
    }
}

fn solve_part2(input: &str) -> String {
    let instructions = read_instructions(input);

    let mut sprite_position: isize = 1;
    let mut sprite_positions: Vec<isize> = Vec::new();
    for instr in instructions.iter() {
        match instr {
            Instr::Noop => {
                sprite_positions.push(sprite_position);
            }
            Instr::Addx(y) => {
                sprite_positions.push(sprite_position);
                sprite_positions.push(sprite_position);
                sprite_position += y;
            }
        }
    }
    sprite_positions
        .into_iter()
        .enumerate()
        .map(|(i, pos)| -> char {
            let crt_pos: isize = (i as isize) % 40;
            //println!("{} {} {} ({}) ", i, crt_pos, pos, (crt_pos - pos).abs());
            if (crt_pos - pos).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<String>()
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
    print_screen("Answer 2", &answer2);
}

#[cfg(test)]
mod tests_day10 {
    use super::*;

    #[test]
    fn test1_1() {
        assert_eq!(solve_part1(EXAMPLE1), 13140);
    }

    #[test]
    fn test_instr_1() {
        assert_eq!(Instr::from_str("noop"), Instr::Noop);
    }

    #[test]
    fn test_instr_2() {
        assert_eq!(Instr::from_str("addx 314"), Instr::Addx(314));
    }

    #[test]
    fn test2_1() {
        let candidate = solve_part2(EXAMPLE1);

        let example1_answer = EXAMPLE1_SCREEN
            .chars()
            .filter(|c| *c != '\n')
            .collect::<String>();
        if candidate != example1_answer {
            print_screen("candidate", &candidate);
            print_screen("answer", &example1_answer);
            assert_eq!(candidate, example1_answer);
        }
    }

    const EXAMPLE1: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const EXAMPLE1_SCREEN: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
}
