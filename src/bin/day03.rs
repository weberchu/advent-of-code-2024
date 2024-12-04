use regex::Regex;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day03.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> i64 {
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;

    for line in input.lines() {
        let captures = mul_pattern.captures_iter(line);
        for capture in captures {
            let num1: i64 = capture.get(1).unwrap().as_str().parse().unwrap();
            let num2: i64 = capture.get(2).unwrap().as_str().parse().unwrap();
            let product = num1 * num2;
            result += product;
        }
    }

    result
}

fn part2(input: &str) -> i64 {
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut result = 0;
    let mut is_do = true;

    for line in input.lines() {
        let captures = mul_pattern.captures_iter(line);
        for capture in captures {
            if capture.get(3).is_some() {
                is_do = true;
            } else if capture.get(4).is_some() {
                is_do = false;
            } else if is_do {
                let num1: i64 = capture.get(1).unwrap().as_str().parse().unwrap();
                let num2: i64 = capture.get(2).unwrap().as_str().parse().unwrap();
                let product = num1 * num2;
                result += product;
            }
        }
    }

    result
}
