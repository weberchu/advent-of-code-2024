use std::collections::HashMap;
use std::iter::Map;

fn main() {
    let input = include_str!("../../input/day01.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut total_distance = 0;
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let split: Vec<&str> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();

        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    for i in 0..left.len() {
        let distance = (left[i] - right[i]).abs();
        total_distance += distance;
    }

    println!("part 1: {}", total_distance);
}

fn part2(input: &str) {
    let mut total_similarity = 0;
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let split: Vec<&str> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();

        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    let mut occurrence_map: HashMap<i32, i32> = HashMap::new();

    for l in left {
        if let Some(occurrence) = occurrence_map.get(&l) {
            total_similarity += l * occurrence;
        } else {
            let occurrence = right.iter()
                .filter(|&&r| r == l)
                .count();
            let occurrence = occurrence as i32;
            occurrence_map.insert(l, occurrence);

            total_similarity += l * occurrence;
        }
    }

    println!("part 2: {}", total_similarity);
}

