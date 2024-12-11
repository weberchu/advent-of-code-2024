use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day11.txt").trim();

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> i64 {
    let stones: Vec<i64> = input.split(" ").map(|n| n.parse().unwrap()).collect();

    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
    let total_stones = stones.iter().map(|&stone| stones_len_after_blink(&mut cache, stone, 25)).sum();

    total_stones
}

fn part2(input: &str) -> i64 {
    let stones: Vec<i64> = input.split(" ").map(|n| n.parse().unwrap()).collect();

    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
    let total_stones = stones.iter().map(|&stone| stones_len_after_blink(&mut cache, stone, 75)).sum();

    total_stones
}

fn stones_len_after_blink(cache: &mut HashMap<(i64, i64), i64>, stone: i64, times: i64) -> i64 {
    if let Some(&result) = cache.get(&(stone, times)) {
        return result;
    }

    if times == 0 {
        return 1;
    }

    let result = if stone == 0 {
        stones_len_after_blink(cache, 1, times - 1)
    } else {
        let stone_in_string = stone.to_string();
        if stone_in_string.len() % 2 == 0 {
            let half_len = stone_in_string.len() / 2;
            let first_half: i64 = stone_in_string[..half_len].parse().unwrap();
            let second_half: i64 = stone_in_string[half_len..].parse().unwrap();
            stones_len_after_blink(cache, first_half, times - 1) + stones_len_after_blink(cache, second_half, times - 1)
        } else {
            stones_len_after_blink(cache, stone * 2024, times - 1)
        }
    };

    cache.insert((stone, times), result);

    result
}
