use regex::Regex;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day17.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

struct Register {
    a: u64,
    b: u64,
    c: u64,
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let a: u64 = read_register(lines.next().unwrap());
    let b: u64 = read_register(lines.next().unwrap());
    let c: u64 = read_register(lines.next().unwrap());
    let register = Register { a, b, c };
    lines.next();
    let program: Vec<u64> = read_program(lines.next().unwrap());

    run_program(register, &program)
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    lines.next();
    let b: u64 = read_register(lines.next().unwrap());
    let c: u64 = read_register(lines.next().unwrap());
    lines.next();
    let program: Vec<u64> = read_program(lines.next().unwrap());
    let program_str: Vec<String> = program.iter().map(|i| i.to_string()).collect();
    let program_str_joined = program_str.join(",");

    let mut indices_to_try = vec![0_u64];

    let mut possible_answers = Vec::new();

    // Digits from the end of the output is a repeating pattern when the output gets longer
    // So we are reverse engineering from the end and find out what the next 8 `a` will produce the
    // same ending with 1 more digit on the left
    while !indices_to_try.is_empty() {
        let index_to_try = indices_to_try.remove(0);
        for a in index_to_try..index_to_try+8 {
            let register = Register { a, b, c };
            let output = run_program(register, &program);

            if program_str_joined == output {
                possible_answers.push(a);
            } else if program_str_joined.ends_with(&output) {
                indices_to_try.push(a * 8);
            }
        }
    }

    possible_answers.iter().min().unwrap().to_owned()
}

fn run_program(register: Register, program: &[u64]) -> String {
    let mut register = register;
    let combo_operand = |opcode: u64, register: &Register| -> u64 {
        match opcode {
            0..=3 => opcode,
            4 => register.a,
            5 => register.b,
            6 => register.c,
            _ => panic!("invalid combo opcode: {}", opcode),
        }
    };

    let mut pointer = 0;
    let mut output = Vec::new();
    while pointer < program.len() - 1 {
        let opcode = program[pointer];
        match opcode {
            0 => {
                let operand = combo_operand(program[pointer + 1], &register);
                register.a /= 2_u64.pow(operand as u32);
                pointer += 2;
            },
            1 => {
                let operand = program[pointer + 1];
                register.b ^= operand;
                pointer += 2;
            },
            2 => {
                let operand = combo_operand(program[pointer + 1], &register);
                register.b = operand % 8;
                pointer += 2;
            },
            3 => {
                if register.a == 0 {
                    pointer += 2;
                } else {
                    let operand = program[pointer + 1];
                    pointer = operand as usize;
                }
            },
            4 => {
                register.b ^= register.c;
                pointer += 2;
            },
            5 => {
                let operand = combo_operand(program[pointer + 1], &register);
                output.push((operand % 8).to_string());
                pointer += 2;
            },
            6 => {
                let operand = combo_operand(program[pointer + 1], &register);
                register.b = register.a / 2_u64.pow(operand as u32);
                pointer += 2;
            },
            7 => {
                let operand = combo_operand(program[pointer + 1], &register);
                register.c = register.a / 2_u64.pow(operand as u32);
                pointer += 2;
            },
            _ => panic!("invalid opcode: {}", opcode),
        }
    }

    output.join(",")
}

fn read_register(register_line: &str) -> u64 {
    let register_pattern = Regex::new(r"Register [ABC]: (\d+)").unwrap();
    register_pattern
        .captures(register_line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn read_program(program_line: &str) -> Vec<u64> {
    let program_pattern = Regex::new(r"Program: ([\d,]+)").unwrap();
    program_pattern
        .captures(program_line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect()
}
