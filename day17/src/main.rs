#![allow(dead_code)]

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;

#[derive(Debug)]
enum Push {
    Left,
    Right,
}

impl Push {
    fn from_str(s: &str) -> Vec<Push> {
        s.trim_end()
            .chars()
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

    fn inverse(&self) -> Push {
        match self {
            Push::Left => Push::Right,
            Push::Right => Push::Left,
        }
    }
}

#[derive(Copy, Clone)]
enum RockKind {
    // ####
    HorzLine = 0b1111,
    // #
    // #
    // #
    // #
    VertLine = 0b1000 | 0b1000 << 4 | 0b1000 << 8 | 0b1000 << 12,
    //  #
    // ###
    //  #
    Plus = 0b0100 | 0b1110 << 4 | 0b0100 << 8,
    //   #
    //   #
    // ###
    RevL = 0b1110 | 0b0010 << 4 | 0b0010 << 8,
    // ##
    // ##
    Square = 0b1100 | 0b1100 << 4,
}

const ROCK_KINDS: [RockKind; 5] = [
    RockKind::HorzLine,
    RockKind::Plus,
    RockKind::RevL,
    RockKind::VertLine,
    RockKind::Square,
];

struct Rock {
    x: u8,
    y: usize,
    rows: u16,
}

impl fmt::Debug for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "({}, {})", self.x, self.y)?;
        writeln!(f, "  {:08b}", ((self.row(3)) << 4) >> self.x)?;
        writeln!(f, "  {:08b}", ((self.row(2)) << 4) >> self.x)?;
        writeln!(f, "  {:08b}", ((self.row(1)) << 4) >> self.x)?;
        write!(f, "  {:08b}", ((self.row(0)) << 4) >> self.x)
    }
}

impl Rock {
    fn new(x: u8, y: usize, rows: u16) -> Rock {
        Rock { x, y, rows }
    }

    fn from_kind(x: u8, y: usize, kind: RockKind) -> Rock {
        Rock::new(x, y, kind as u16)
    }

    fn row(&self, i: usize) -> u8 {
        assert!(i < 4);
        (self.rows >> (i * 4) & 0b1111) as u8
    }

    fn shifted_row(&self, i: usize) -> Option<u8> {
        assert!(self.x < 7);
        let r8 = self.row(i) << 4;
        let r16 = (r8 as u16) << 7;
        if ((r16 >> self.x) & 0b11111111) == 0 {
            Some(r8 >> self.x)
        } else {
            None
        }
    }

    fn apply_push(&mut self, p: &Push) {
        match p {
            Push::Left => {
                if self.x > 0 {
                    self.x -= 1;
                }
            }
            Push::Right => {
                if self.x < 6 {
                    self.x += 1;
                }
            }
        }
    }

    fn overlaps_tower(&self, tower: &Tower) -> bool {
        assert!(self.y > 0);
        for r in 0..self.row_count() {
            if let Some(rock_row) = self.shifted_row(r) {
                let y = r + self.y;
                if y >= tower.row_count() {
                    continue;
                }
                let tower_row = tower.row(y);
                if (tower_row & rock_row) != 0 {
                    return true;
                }
            } else {
                return true;
            }
        }
        return false;
    }

    fn row_count(&self) -> usize {
        for i in 0..4 {
            if (0xf000 >> (4 * i)) & self.rows != 0 {
                return 4 - i;
            }
        }
        0
    }
}

struct Tower {
    rows: Vec<u8>,
    y_offset: usize,
}

impl Tower {
    fn new() -> Tower {
        Tower {
            rows: vec![0xff],
            y_offset: 0,
        }
    }

    fn clone_top(&self, rock_count: usize, y_offset: usize) -> Tower {
        assert!(rock_count <= self.rows.len());
        let top_start_index = self.rows.len() - rock_count;
        let mut top_rows = Vec::new();
        top_rows.extend_from_slice(&self.rows[top_start_index..]);
        Tower {
            rows: top_rows,
            y_offset,
        }
    }

    fn row_count(&self) -> usize {
        self.rows.len() + self.y_offset
    }

    fn height(&self) -> usize {
        self.row_count() - 1
    }

    fn row(&self, y: usize) -> u8 {
        assert!(y >= self.y_offset);
        assert!(y <= self.rows.len() + self.y_offset);
        self.rows[y - self.y_offset]
    }

    fn add_rock(&mut self, rock: &Rock) {
        assert!(rock.y > 0);
        assert!(rock.y >= self.y_offset);
        assert!((rock.y - self.y_offset) <= self.rows.len() + 1);
        for r in 0..rock.row_count() {
            let rock_row = rock.shifted_row(r).unwrap();
            let y = (r + rock.y) - self.y_offset;
            if y < self.rows.len() {
                self.rows[y] |= rock_row;
            } else {
                self.rows.push(rock_row);
            }
        }
    }
}

impl fmt::Debug for Tower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "height {}, y_offset {}", self.height(), self.y_offset)?;
        for (y, r) in self.rows.iter().enumerate().rev() {
            writeln!(f, "{:03} {:08b}", y + self.y_offset, r)?;
        }
        Ok(())
    }
}

#[derive(Hash, PartialEq, Eq)]
struct HeightRecordKey {
    key: u16,
}

impl HeightRecordKey {
    fn new() -> HeightRecordKey {
        HeightRecordKey { key: 0 }
    }

    fn add_x(&mut self, x: u8, rock_index: usize) {
        assert!(x < 7);
        assert!(rock_index <= 5);
        let shift = rock_index % 5;
        self.key |= (x as u16 & 0b111) << shift * 3;
    }
}

impl fmt::Debug for HeightRecordKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:1}{:1}{:1}{:1}{:1}",
            (self.key >> 3 * 0) & 0b111,
            (self.key >> 3 * 1) & 0b111,
            (self.key >> 3 * 2) & 0b111,
            (self.key >> 3 * 3) & 0b111,
            (self.key >> 3 * 4) & 0b111
        )
    }
}

struct RocksHeightRecord {
    r: usize,
    height: usize,
    push_index: usize,
    height_diff: usize,
    rocks_since_last: usize,
    match_count: usize,
}

impl RocksHeightRecord {
    fn new(r: usize, height: usize, push_index: usize) -> RocksHeightRecord {
        RocksHeightRecord {
            r,
            height,
            push_index,
            height_diff: 0,
            rocks_since_last: 0,
            match_count: 1,
        }
    }

    fn new_diff(
        r: usize,
        height: usize,
        push_index: usize,
        prev_record: &RocksHeightRecord,
    ) -> RocksHeightRecord {
        let rocks_since_last = r - prev_record.r;
        let height_diff = height - prev_record.height;
        let match_count = if height_diff == prev_record.height_diff
            && rocks_since_last == prev_record.rocks_since_last
            && push_index == prev_record.push_index
        {
            prev_record.match_count + 1
        } else {
            1
        };
        RocksHeightRecord {
            r,
            height,
            push_index,
            height_diff,
            rocks_since_last,
            match_count,
        }
    }
}

impl fmt::Debug for RocksHeightRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:06} height {:06}, push_index {:06}, height_diff {:04} \
            , rocks_since_last {:04}, match_count {}",
            self.r,
            self.height,
            self.push_index,
            self.height_diff,
            self.rocks_since_last,
            self.match_count
        )
    }
}

fn drop_rock(
    rock_kind: &RockKind,
    tower: &mut Tower,
    pushes: &[Push],
    start_push_index: usize,
) -> (Rock, usize) {
    let mut rock = Rock::from_kind(2, tower.row_count() + 3, *rock_kind);

    let mut push_index = start_push_index;
    loop {
        let push = &pushes[push_index];
        push_index = (push_index + 1) % pushes.len();
        rock.apply_push(push);
        if rock.overlaps_tower(&tower) {
            rock.apply_push(&push.inverse());
        }
        rock.y -= 1;
        if rock.y == 0 || rock.overlaps_tower(&tower) {
            rock.y += 1;
            tower.add_rock(&rock);
            break;
        }
    }

    (rock, push_index)
}

fn get_tower_height_from(
    mut tower: Tower,
    pushes: &[Push],
    mut push_index: usize,
    start_r: usize,
    rock_count: usize,
) -> usize {
    for r in start_r..rock_count {
        let rock_index = r % ROCK_KINDS.len();
        let rock_kind = &ROCK_KINDS[rock_index];
        let (_, new_push_index) = drop_rock(rock_kind, &mut tower, pushes, push_index);
        push_index = new_push_index;
    }

    tower.height()
}

fn get_tower_height(pushes: &[Push], rock_count: usize) -> usize {
    let mut tower = Tower::new();
    let mut push_index = 0;
    let mut record_key = HeightRecordKey::new();
    let mut height_records: HashMap<HeightRecordKey, RocksHeightRecord> = HashMap::new();
    for r in 0..rock_count {
        let rock_index = r % ROCK_KINDS.len();
        let rock_kind = &ROCK_KINDS[rock_index];
        let (rock, new_push_index) = drop_rock(rock_kind, &mut tower, pushes, push_index);
        push_index = new_push_index;

        record_key.add_x(rock.x, rock_index);
        if rock_index == ROCK_KINDS.len() - 1 {
            let record = if let Some(prev_record) = height_records.get(&record_key) {
                RocksHeightRecord::new_diff(r, tower.height(), push_index, &prev_record)
            } else {
                RocksHeightRecord::new(r, tower.height(), push_index)
            };
            if record.match_count >= 10 {
                let rocks_left = rock_count - r;
                let jumps = rocks_left / record.rocks_since_last;
                let new_r = r + 1 + jumps * record.rocks_since_last;
                let y_offset =
                    (tower.height() - record.height_diff) + jumps * record.height_diff + 1;
                return get_tower_height_from(
                    tower.clone_top(record.height_diff, y_offset),
                    pushes,
                    push_index,
                    new_r,
                    rock_count,
                );
            } else {
                height_records.insert(record_key, record);
                record_key = HeightRecordKey::new();
            }
        }
    }

    tower.row_count() - 1
}

fn solve_part1(input: &str) -> usize {
    let pushes = Push::from_str(input);

    get_tower_height(&pushes[..], 2022)
}

fn solve_part2(input: &str) -> usize {
    let pushes = Push::from_str(input);

    get_tower_height(&pushes[..], 1000000000000)
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
    fn test1_get_tower_height_1() {
        let pushes = Push::from_str(EXAMPLE1);

        assert_eq!(get_tower_height(&pushes[..], 1), 1);
        assert_eq!(get_tower_height(&pushes[..], 2), 4);
        assert_eq!(get_tower_height(&pushes[..], 3), 6);
        assert_eq!(get_tower_height(&pushes[..], 4), 7);
        assert_eq!(get_tower_height(&pushes[..], 5), 9);
    }

    #[test]
    fn test1_get_tower_height_2() {
        let pushes = Push::from_str(EXAMPLE1);

        assert_eq!(get_tower_height(&pushes[..], 2022), 3068);
    }

    #[test]
    fn test1_get_tower_height_from_1() {
        let pushes = Push::from_str(EXAMPLE1);

        assert_eq!(
            get_tower_height_from(Tower::new(), &pushes[..], 0, 0, 2022),
            3068
        );
    }

    #[test]
    fn test1_rock_row_1() {
        assert_eq!(Rock::new(0, 0, 0b1111).row(0), 0b1111);
        assert_eq!(Rock::new(0, 0, 0b1111).row(1), 0b0000);
        assert_eq!(Rock::new(0, 0, 0b1111).row(2), 0b0000);
        assert_eq!(Rock::new(0, 0, 0b1111).row(3), 0b0000);
    }

    #[test]
    fn test1_rock_row_2() {
        assert_eq!(Rock::from_kind(0, 0, RockKind::Plus).row(0), 0b0100);
        assert_eq!(Rock::from_kind(0, 0, RockKind::Plus).row(1), 0b1110);
        assert_eq!(Rock::from_kind(0, 0, RockKind::Plus).row(2), 0b0100);
        assert_eq!(Rock::from_kind(0, 0, RockKind::Plus).row(3), 0b0000);
    }

    #[test]
    fn test1_rock_shifted_row_1() {
        assert_eq!(Rock::new(0, 0, 0b1111).shifted_row(0), Some(0b11110000));
        assert_eq!(Rock::new(1, 0, 0b1111).shifted_row(0), Some(0b01111000));
        assert_eq!(Rock::new(2, 0, 0b1111).shifted_row(0), Some(0b00111100));
        assert_eq!(Rock::new(3, 0, 0b1111).shifted_row(0), Some(0b00011110));
        assert_eq!(Rock::new(4, 0, 0b1110).shifted_row(0), Some(0b00001110));
        assert_eq!(Rock::new(5, 0, 0b1100).shifted_row(0), Some(0b00000110));
        assert_eq!(Rock::new(6, 0, 0b1000).shifted_row(0), Some(0b00000010));
    }

    #[test]
    fn test1_rock_shifted_row_2() {
        assert_eq!(Rock::new(4, 0, 0b0001).shifted_row(0), None);
        assert_eq!(Rock::new(5, 0, 0b0010).shifted_row(0), None);
        assert_eq!(Rock::new(6, 0, 0b0100).shifted_row(0), None);
    }

    #[test]
    fn test1_rock_row_count_1() {
        assert_eq!(Rock::new(0, 0, 0).row_count(), 0);
        assert_eq!(Rock::from_kind(0, 0, RockKind::HorzLine).row_count(), 1);
        assert_eq!(Rock::from_kind(0, 0, RockKind::VertLine).row_count(), 4);
        assert_eq!(Rock::from_kind(0, 0, RockKind::Plus).row_count(), 3);
        assert_eq!(Rock::from_kind(0, 0, RockKind::Square).row_count(), 2);
    }

    #[test]
    fn test1_rock_push_overlap_1() {
        let mut rock = Rock::from_kind(4, 1, RockKind::RevL);
        let tower = Tower::new();
        assert_eq!(rock.overlaps_tower(&tower), false);
        rock.apply_push(&Push::Right);
        assert_eq!(rock.overlaps_tower(&tower), true);
    }

    #[test]
    fn test2_1() {
        assert_eq!(solve_part2(EXAMPLE1), 1514285714288);
    }

    #[test]
    fn test2_tower_clone_top_1() {
        let mut tower = Tower::new();
        tower.add_rock(&Rock::from_kind(0, 1, RockKind::RevL));
        tower.add_rock(&Rock::from_kind(0, 4, RockKind::RevL));
        assert_eq!(tower.height(), 6);

        let top = tower.clone_top(2, tower.row_count() - 2);

        assert_eq!(top.height(), 6);
    }
}
