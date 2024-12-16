use std::collections::{HashMap, HashSet};
use std::time::Instant;

const MOVE_COST: i32 = 1;
const TURN_COST: i32 = 1000;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
    N, E, S, W
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        }
    }

    fn turns_required(&self, other: &Direction) -> i32 {
        match (self, other) {
            (Direction::N, Direction::S) => 2,
            (Direction::S, Direction::N) => 2,
            (Direction::E, Direction::W) => 2,
            (Direction::W, Direction::E) => 2,
            (Direction::N, Direction::N) => 0,
            (Direction::S, Direction::S) => 0,
            (Direction::E, Direction::E) => 0,
            (Direction::W, Direction::W) => 0,
            _ => 1,
        }
    }

    fn cmp_value(&self) -> i32 {
        match self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
        }
    }
}

type CoordWithDirection = ((i32, i32), Direction);

fn main() {
    let input = include_str!("../../input/day16.txt");

    let start = Instant::now();
    let (p1_ans, p2_ans) = part1_and_2(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1_and_2(input: &str) -> (i32, i32) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = map.len();
    let start = find(&map, 'S');
    let goal = find(&map, 'E');

    // cost_map. key: coordinates, value: a pair of cost and coordinates on all possible paths with that cost
    let mut cost_map: HashMap<CoordWithDirection, (i32, HashSet<(i32, i32)>)>= HashMap::new();
    let mut processing_queue: Vec<CoordWithDirection> = Vec::new();

    cost_map.insert((start, Direction::E), (0, HashSet::from([start])));
    processing_queue.push(((start.0, start.1), Direction::E));

    while !processing_queue.is_empty() {
        processing_queue.sort_by(|a, b| {
            let (cost_a, _) = cost_map.get(a).unwrap();
            let (cost_b, _) = cost_map.get(b).unwrap();
            if cost_a == cost_b {
                let cmp_value_a = (a.0.1 * height as i32 + a.0.0) * 10 + a.1.cmp_value();
                let cmp_value_b = (b.0.1 * height as i32 + b.0.0) * 10 + b.1.cmp_value();
                cmp_value_a.cmp(&cmp_value_b)
            } else {
                cost_a.cmp(cost_b)
            }
        });

        let (pos_to_process, facing_direction) = processing_queue.remove(0);

        let neighbours = [Direction::N, Direction::E, Direction::S, Direction::W]
            .map(|d| {
                let offset = d.offset();
                ((pos_to_process.0 + offset.0, pos_to_process.1 + offset.1), d)
            });
        for (neighbour, neighbour_direction) in neighbours {
            match map[neighbour.1 as usize][neighbour.0 as usize] {
                '.' | 'E' => {
                    let turns_cost = facing_direction.turns_required(&neighbour_direction);
                    let (current_cost, current_path) = cost_map.get(&(pos_to_process, facing_direction.clone())).unwrap();
                    let new_neighbour_cost = current_cost + MOVE_COST + TURN_COST * turns_cost;
                    let (original_neighbour_cost, original_neighbour_path) = if let Some((original_neighbour_cost, original_neighbour_path)) = cost_map.get(&(neighbour, neighbour_direction.clone())) {
                        (original_neighbour_cost.to_owned(), original_neighbour_path)
                    } else {
                        (i32::MAX, &HashSet::new())
                    };

                    if new_neighbour_cost > original_neighbour_cost {
                        continue;
                    } else {
                        let mut coordinates_on_path = current_path.clone();
                        coordinates_on_path.insert(neighbour);

                        if new_neighbour_cost == original_neighbour_cost {
                            // coordinates from both old and new paths are on the lowest cost path
                            coordinates_on_path = coordinates_on_path.union(original_neighbour_path).map(|i| i.to_owned()).collect();
                        }

                        cost_map.insert((neighbour, neighbour_direction.clone()), (new_neighbour_cost, coordinates_on_path));
                        processing_queue.push((neighbour, neighbour_direction.clone()));
                    }
                }
                _ => {}
            }
        }
    }

    let goal_cost = [Direction::N, Direction::E, Direction::S, Direction::W]
        .map(|direction| {
            if let Some((cost, _)) = cost_map.get(&(goal, direction)) {
                cost.to_owned()
            } else {
                i32::MAX
            }
        })
        .iter()
        .min()
        .unwrap()
        .to_owned();

    let coordinates_in_cheapest_path = [Direction::N, Direction::E, Direction::S, Direction::W]
        .iter()
        .filter(|&direction| {
            if let Some((cost, _)) = cost_map.get(&(goal, direction.clone())) {
                *cost == goal_cost
            } else {
                false
            }
        })
        .flat_map(|direction| cost_map.get(&(goal, direction.clone())).unwrap().1.clone())
        .count();

    (goal_cost.to_owned(), coordinates_in_cheapest_path as i32)
}

fn find(map: &[Vec<char>], target: char) -> (i32, i32) {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if target == *c {
                return (x as i32, y as i32);
            }
        }
    }

    panic!("not found");
}
