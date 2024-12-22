#![allow(clippy::comparison_chain)]
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day21.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

#[derive(Clone)]
enum Keypad {
    Numeric (char),
    Directional (char),
}

impl Keypad {
    fn sequences_for_button(&mut self, button: char) -> Vec<String> {
        match self {
            Keypad::Numeric(current_key) => {
                let self_row = Self::numeric_row(current_key);
                let self_column = Self::numeric_column(current_key);
                let button_row = Self::numeric_row(&button);
                let button_column = Self::numeric_column(&button);
                let row_diff = button_row - self_row;
                let column_diff = button_column - self_column;

                let mut sequences = HashSet::new();

                // <> and then ^v
                if !(self_row == 3 && button_column == 0) {
                    let mut sequence = String::new();
                    if column_diff < 0 {
                        sequence.push_str(&"<".repeat(column_diff.unsigned_abs() as usize));
                    } else if column_diff > 0 {
                        sequence.push_str(&">".repeat(column_diff as usize));
                    }
                    if row_diff < 0 {
                        sequence.push_str(&"^".repeat(row_diff.unsigned_abs() as usize));
                    } else if row_diff > 0 {
                        sequence.push_str(&"v".repeat(row_diff as usize));
                    }
                    sequence.push('A');

                    sequences.insert(sequence);
                }

                // ^v and then <>
                if !(self_column == 0 && button_row == 3) {
                    let mut sequence = String::new();
                    if row_diff < 0 {
                        sequence.push_str(&"^".repeat(row_diff.unsigned_abs() as usize));
                    } else if row_diff > 0 {
                        sequence.push_str(&"v".repeat(row_diff as usize));
                    }
                    if column_diff < 0 {
                        sequence.push_str(&"<".repeat(column_diff.unsigned_abs() as usize));
                    } else if column_diff > 0 {
                        sequence.push_str(&">".repeat(column_diff as usize));
                    }
                    sequence.push('A');

                    sequences.insert(sequence);
                }

                *current_key = button;

                sequences.iter().cloned()
                    .collect()
            },
            Keypad::Directional(current_key) => {
                let self_row = Self::directional_row(current_key);
                let self_column = Self::directional_column(current_key);
                let button_row = Self::directional_row(&button);
                let button_column = Self::directional_column(&button);
                let row_diff = button_row - self_row;
                let column_diff = button_column - self_column;

                let mut sequences = HashSet::new();

                // <> and then ^v
                if !(self_row == 0 && button_column == 0) {
                    let mut sequence = String::new();
                    if column_diff < 0 {
                        sequence.push_str(&"<".repeat(column_diff.unsigned_abs() as usize));
                    } else if column_diff > 0 {
                        sequence.push_str(&">".repeat(column_diff as usize));
                    }
                    if row_diff < 0 {
                        sequence.push_str(&"^".repeat(row_diff.unsigned_abs() as usize));
                    } else if row_diff > 0 {
                        sequence.push_str(&"v".repeat(row_diff as usize));
                    }
                    sequence.push('A');

                    sequences.insert(sequence);
                }

                // ^v and then <>
                if !(self_column == 0 && button_row == 0) {
                    let mut sequence = String::new();
                    if row_diff < 0 {
                        sequence.push_str(&"^".repeat(row_diff.unsigned_abs() as usize));
                    } else if row_diff > 0 {
                        sequence.push_str(&"v".repeat(row_diff as usize));
                    }
                    if column_diff < 0 {
                        sequence.push_str(&"<".repeat(column_diff.unsigned_abs() as usize));
                    } else if column_diff > 0 {
                        sequence.push_str(&">".repeat(column_diff as usize));
                    }
                    sequence.push('A');

                    sequences.insert(sequence);
                }

                *current_key = button;

                sequences.iter().cloned().collect()
            },
        }
    }

    fn numeric_column(button: &char) -> i32 {
        match button {
            '7' | '4' | '1' => 0,
            '8' | '5' | '2' | '0' => 1,
            '9' | '6' | '3' | 'A' => 2,
            _ => panic!("unknown button"),
        }
    }

    fn numeric_row(button: &char) -> i32 {
        match button {
            '7' | '8' | '9' => 0,
            '4' | '5' | '6' => 1,
            '1' | '2' | '3' => 2,
            '0' | 'A' => 3,
            _ => panic!("unknown button"),
        }
    }

    fn directional_column(button: &char) -> i32 {
        match button {
            '<' => 0,
            '^' | 'v' => 1,
            'A' | '>' => 2,
            _ => panic!("unknown button"),
        }
    }

    fn directional_row(button: &char) -> i32 {
        match button {
            '^' | 'A' => 0,
            '<' | 'v' | '>' => 1,
            _ => panic!("unknown button"),
        }
    }
}

fn find_key_sequence_len(sequence: String, keypads: &[Keypad], cache: &mut HashMap<(String, usize), usize>) -> usize {
    if let Some(&cache_len) = cache.get(&(sequence.clone(), keypads.len())) {
        return cache_len
    }

    let mut keypads = keypads.to_owned();
    let mut keypad = keypads.remove(0);
    let len = sequence.chars().map(|key| {
        let next_sequences = keypad.sequences_for_button(key);
        next_sequences.iter().map(|next_sequence| {
            if keypads.is_empty() {
                next_sequence.len()
            } else {
                find_key_sequence_len(next_sequence.clone(), &keypads, cache)
            }
        }).min().unwrap()
    }).sum();

    cache.insert((sequence, keypads.len() + 1), len);

    len
}

fn part1(input: &str) -> usize {
    let mut cache = HashMap::new();
    input.lines().map(|line| {
        let shortest_sequence_len = find_key_sequence_len(
            line.to_string(),
            &[
                Keypad::Numeric ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
            ],
            &mut cache
        );
        line[0..line.len() - 1].parse::<usize>().unwrap() * shortest_sequence_len
    }).sum()
}

fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();
    input.lines().map(|line| {
        let shortest_sequence_len = find_key_sequence_len(
            line.to_string(),
            &

                [
                Keypad::Numeric ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
                Keypad::Directional ('A'),
            ],
            &mut cache
        );
        line[0..line.len() - 1].parse::<usize>().unwrap() * shortest_sequence_len


    }).sum()
}
