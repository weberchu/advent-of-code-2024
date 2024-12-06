use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day06.txt");

    let start = Instant::now();
    let (p1_ans, p2_ans) = part1_and_2(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

fn part1_and_2(input: &str) -> (usize, usize) {
    let map: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut guard_start_position = (0, 0);
    for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '^' {
                guard_start_position = (x as i32, y as i32);
                break;
            }
        }
    }

    let visited = patrol(&map, &guard_start_position).expect("should not dead loop without extra obstacle");

    let mut obstacles = visited.clone();
    obstacles.remove(&guard_start_position);
    let mut stuck_count = 0;
    for obstacle in obstacles {
        let mut map_with_obstacle: Vec<Vec<char>>= map.to_vec();
        map_with_obstacle[obstacle.1 as usize][obstacle.0 as usize] = '#';

        let visited = patrol(&map_with_obstacle, &guard_start_position);
        if visited.is_none() {
            stuck_count += 1;
        }
    }

    (visited.len(), stuck_count as usize)
}

fn patrol(map: &[Vec<char>], guard_start_position: &(i32, i32)) -> Option<HashSet<(i32, i32)>> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut guard = *guard_start_position;
    let mut direction = Direction::North;
    let mut visited = HashSet::new();
    let mut visited_with_direction = HashSet::new();

    loop {
        visited.insert(guard);
        visited_with_direction.insert((guard, direction));

        let offset = direction.offset();
        let next_position = (guard.0 + offset.0, guard.1 + offset.1);

        if !is_in_map(height, width, &next_position) {
            break;
        }

        if map[next_position.1 as usize][next_position.0 as usize] == '#' {
            direction = direction.rotate();
        } else {
            guard = next_position
        };

        if visited_with_direction.contains(&(guard, direction)) {
            return None;
        }
    }

    Some(visited)
}

fn is_in_map(height: i32, width: i32, guard: &(i32, i32)) -> bool {
    guard.0 >= 0 && guard.0 < width && guard.1 >= 0 && guard.1 < height
}
