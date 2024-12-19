use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day19.txt");

    let start = Instant::now();
    let (p1_ans, p2_ans) = part1_and_2(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

fn part1_and_2(input: &str) -> (usize, usize) {
    let mut lines = input.lines();

    let towels: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let mut possibility_cache = HashMap::new();

    lines.next();
    let designs: Vec<&str> = lines.collect();

    let mut total_possible_design: usize = 0;
    let mut total_possibility = 0;
    designs.iter().for_each(|&pattern| {
        let count = is_possible(pattern, &towels, &mut possibility_cache);
        if count > 0 {
            total_possible_design += 1;
        }
        total_possibility += count;
    });

    (total_possible_design, total_possibility)
}

fn is_possible<'a>(pattern: &'a str, towels: &[&str], possibility_cache: &mut HashMap<&'a str, usize>) -> usize {

    if let Some(&possible_count) = possibility_cache.get(pattern) {
        return possible_count;
    }

    towels.iter().map(|&towel| {
        if pattern == towel {
            possibility_cache
                .entry(pattern)
                .and_modify(|c| *c += 1)
                .or_insert(1);
            1
        } else if let Some(remaining_pattern) = pattern.strip_prefix(towel) {
            let count = is_possible(remaining_pattern, towels, possibility_cache);
            possibility_cache
                .entry(pattern)
                .and_modify(|c| *c += count)
                .or_insert(count);
            count
        } else {
            0
        }
    }).sum()
}
