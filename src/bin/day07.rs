use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day07.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let test_value = split[0].parse().unwrap();
        let numbers: Vec<i64> = split[1].split(" ").map(|n| n.parse().unwrap()).collect();

        for operator_bits in 0..2u32.pow(numbers.len() as u32 - 1) {
            if match_1(&numbers, operator_bits, test_value) {
                total += test_value;
                break;
            }
        }
    }

    total
}

/// operator bits - 0 is +, 1 is *
fn match_1(numbers: &[i64], operator_bits: u32, test_value: i64) -> bool {
    let mut result = numbers[0];
    for (i, num) in numbers[1..].iter().enumerate() {
        let operator = (operator_bits >> i) & 1;
        if operator == 0 {
            result += num;
        } else {
            result *= num;
        }

        if result > test_value {
            return false;
        }
    }

    result == test_value
}

fn part2(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(": ").collect();
        let test_value = split[0].parse().unwrap();
        let numbers: Vec<i64> = split[1].split(" ").map(|n| n.parse().unwrap()).collect();

        let mut bits_combination = vec![0];

        // generate numbers with groups of 00, 01, 10 but never 11
        // hence 0, 1, 2, 4, 5, 6, 8, 9, 10
        for _ in 0..numbers.len() - 1 {
            bits_combination = bits_combination.iter().flat_map(|bits| {
                let new_bits = bits << 2;
                (0..3).map(|n| {
                    new_bits | n
                }).collect::<Vec<_>>()
            }).collect();
        }

        for operator_bits in bits_combination {
            if match_2(&numbers, operator_bits, test_value) {
                total += test_value;
                break;
            }
        }
    }

    total
}

/// operator bits - group of every 2 bits, 00 is +, 01 is *, 10 is ||, 11 is invalid
fn match_2(numbers: &[i64], operator_bits: u32, test_value: i64) -> bool {
    let mut result = numbers[0];


    for (i, num) in numbers[1..].iter().enumerate() {
        let operator = (operator_bits >> (i*2)) & 3;
        match operator {
            0 => result += num,
            1 => result *= num,
            2 => {
                let num_len = num.to_string().len() as u32;
                result = result * 10_i64.pow(num_len) + num;
            },
            _ => panic!("invalid operator: {}", operator)
        }

        if result > test_value {
            return false;
        }
    }

    result == test_value
}
