use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const MAP_SIZE: i32 = 71;

#[derive(Debug)]
struct CoordinateCost {
    coordinate: (i32, i32),
    cost: i32,
}

fn main() {
    let input = include_str!("../../input/day18.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> i32 {
    let corruption_sequence = corrupted_sequence(input);

    if let Some(value) = shortest_path(&corruption_sequence, 1024) {
        return value;
    }

    panic!("No path found");
}

fn part2(input: &str) -> String {
    let corruption_sequence = corrupted_sequence(input);
    let total_corruption = corruption_sequence.len();

    let mut highest_possible_corruption = 0;
    let mut lowest_impossible_corruption = total_corruption;

    while highest_possible_corruption != lowest_impossible_corruption - 1 {
        let next_corruption_to_try = (highest_possible_corruption + lowest_impossible_corruption) / 2;
        let cost = shortest_path(&corruption_sequence, next_corruption_to_try);
        if cost.is_some() {
            highest_possible_corruption = next_corruption_to_try;
        } else {
            lowest_impossible_corruption = next_corruption_to_try;
        }
    }

    let result: &(i32, i32) = corruption_sequence.get(highest_possible_corruption).unwrap();
    format!("{},{}", result.0, result.1)
}

fn corrupted_sequence(input: &str) -> Vec<(i32, i32)> {
    let corruption_sequence: Vec<(i32, i32)> = input.lines().map(|line| {
        let mut split = line.split(",");
        (split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap())
    }).collect::<Vec<(i32, i32)>>();
    corruption_sequence
}

fn shortest_path(corruption_sequence: &[(i32, i32)], bytes_to_take: usize) -> Option<i32> {
    let corrupted_bytes: HashSet<(i32, i32)> = corruption_sequence[0..bytes_to_take].iter().map(|(x, y)| (*x, *y)).collect();

    let mut processed = HashMap::new();
    let mut to_process = vec![CoordinateCost {
        coordinate: (0, 0),
        cost: 0
    }];

    let goal = (MAP_SIZE - 1, MAP_SIZE - 1);

    while !to_process.is_empty() {
        to_process.sort_by(|cc1, cc2| {
            let cmp = cc1.cost.cmp(&cc2.cost);
            if cmp == Ordering::Equal {
                let hash1 = cc1.coordinate.0 + cc1.coordinate.1 * MAP_SIZE;
                let hash2 = cc2.coordinate.0 + cc2.coordinate.1 * MAP_SIZE;
                hash1.cmp(&hash2)
            } else {
                cmp
            }
        });

        let CoordinateCost { coordinate, cost } = to_process.remove(0);
        if let Some(&processed_cost) = processed.get(&coordinate) {
            if cost >= processed_cost {
                continue;
            }
        }
        processed.insert(coordinate, cost);

        for neighbour in neighbours(&coordinate) {
            if !corrupted_bytes.contains(&neighbour) && !processed.contains_key(&neighbour) {
                let neighbour_cost = cost + 1;
                if neighbour == goal {
                    return Some(neighbour_cost);
                } else {
                    to_process.push(CoordinateCost {
                        coordinate: neighbour,
                        cost: neighbour_cost
                    });
                }
            }
        }
    }

    None
}

fn neighbours(coordinate: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbours = Vec::new();
    for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let x = coordinate.0 + offset.0;
        let y = coordinate.1 + offset.1;
        if (0..MAP_SIZE).contains(&x) && (0..MAP_SIZE).contains(&y) {
            neighbours.push((x, y));
        }
    }
    neighbours
}
