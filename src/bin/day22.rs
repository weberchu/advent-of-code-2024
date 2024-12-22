use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day22.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

fn generate_next(secret: i64) -> i64 {
    let mut next = (secret * 64) ^ secret;
    next %= 16777216;

    next = (next / 32) ^ next;
    next %= 16777216;

    next = (next * 2048) ^ next;
    next %= 16777216;

    next
}

fn generate_times(secret: i64, times: i64) -> i64 {
    (0..times).fold(secret, |acc, _| {
        generate_next(acc)
    })
}

fn generate_sequence(secret: i64, times: i64) -> Vec<i64> {
    let mut secret = secret;
    let mut sequence = vec![secret];
    for _ in 0..times {
        secret = generate_next(secret);
        sequence.push(secret);
    }
    sequence
}

fn part1(input: &str) -> i64 {
    input.lines().map(|line| {
        let secret: i64 = line.parse().unwrap();
        generate_times(secret, 2000)
    }).sum()
}

fn part2(input: &str) -> i64 {
    let monkey_sequence_price_maps = input.lines().map(|line| {
        let price_sequence = generate_sequence(line.parse().unwrap(), 2000)
            .iter()
            .map(|&n| n % 10)
            .collect::<Vec<i64>>();

        let price_differences = price_sequence
            .windows(2)
            .map(|window| {
                window[1] - window[0]
            })
            .collect::<Vec<i64>>();

        let mut monkey_sequence_price_map = HashMap::new();
        for i in 0..price_differences.len() - 4 {
            let monkey_sequence = (
                price_differences[i],
                price_differences[i + 1],
                price_differences[i + 2],
                price_differences[i + 3],
            );
            let price = price_sequence[i + 4];

            monkey_sequence_price_map.entry(monkey_sequence).or_insert(price);
        }

        monkey_sequence_price_map
    }).collect::<Vec<HashMap<(i64, i64, i64, i64), i64>>>();

    let all_monkey_sequences = monkey_sequence_price_maps
        .iter()
        .flat_map(|map| map.keys().cloned())
        .collect::<HashSet<(i64, i64, i64, i64)>>();

    all_monkey_sequences.iter().map(|monkey_sequence| {
        monkey_sequence_price_maps.iter().map(|price_map| {
            price_map.get(monkey_sequence).unwrap_or(&0).to_owned()
        }).sum()
    }).max().unwrap()
}
