use std::time::Instant;
use regex::Regex;

fn main() {
    let input = include_str!("../../input/day13.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> i32 {
    let button_pattern = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut lines = input.lines();
    let mut total = 0;
    while let Some(line) = lines.next() {
        let button_a = button_pattern.captures(line).unwrap();
        let a_x: i32 = button_a.get(1).unwrap().as_str().parse().unwrap();
        let a_y: i32 = button_a.get(2).unwrap().as_str().parse().unwrap();

        let line = lines.next().unwrap();
        let button_b = button_pattern.captures(line).unwrap();
        let b_x: i32 = button_b.get(1).unwrap().as_str().parse().unwrap();
        let b_y: i32 = button_b.get(2).unwrap().as_str().parse().unwrap();

        let line = lines.next().unwrap();
        let prize = prize_pattern.captures(line).unwrap();
        let prize_x: i32 = prize.get(1).unwrap().as_str().parse().unwrap();
        let prize_y: i32 = prize.get(2).unwrap().as_str().parse().unwrap();

        let b: f64 = (a_x * prize_y - a_y * prize_x) as f64 / (a_x * b_y - a_y * b_x) as f64;
        let a: f64 = (prize_x as f64 - (b * b_x as f64)) / a_x as f64;

        if is_whole_number(a) && is_whole_number(b) {
            total += 3 * a as i32 + b as i32;
        }

        lines.next();
    }

    total
}

fn is_whole_number(f: f64) -> bool {
    let diff = f - f.round();
    diff > -0.0000001 && diff < 0.0000001
}

fn part2(input: &str) -> i64 {
    let button_pattern = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut lines = input.lines();
    let mut total = 0;
    while let Some(line) = lines.next() {
        let button_a = button_pattern.captures(line).unwrap();
        let a_x: i64 = button_a.get(1).unwrap().as_str().parse().unwrap();
        let a_y: i64 = button_a.get(2).unwrap().as_str().parse().unwrap();

        let line = lines.next().unwrap();
        let button_b = button_pattern.captures(line).unwrap();
        let b_x: i64 = button_b.get(1).unwrap().as_str().parse().unwrap();
        let b_y: i64 = button_b.get(2).unwrap().as_str().parse().unwrap();

        let line = lines.next().unwrap();
        let prize = prize_pattern.captures(line).unwrap();
        let prize_x: i64 = prize.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000i64;
        let prize_y: i64 = prize.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000i64;

        let b: f64 = (a_x * prize_y - a_y * prize_x) as f64 / (a_x * b_y - a_y * b_x) as f64;
        let a: f64 = (prize_x as f64 - (b * b_x as f64)) / a_x as f64;

        if is_whole_number(a) && is_whole_number(b) {
            total += 3 * a as i64 + b as i64;
        }

        lines.next();
    }

    total
}
