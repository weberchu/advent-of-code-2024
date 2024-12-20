use std::time::Instant;
use std::collections::{VecDeque, HashMap};

fn main() {
    let input = include_str!("../../input/day20.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    race_with_cheat(input, 2)
}

fn part2(input: &str) -> usize {
    race_with_cheat(input, 20)
}

fn race_with_cheat(input: &str, cheat_len: i32) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let time_map = time_map(&map);

    let mut cheats_saved_100_count = 0;

    // cheat
    for (&(from_x, from_y), &from_time) in time_map.iter() {
        if map[from_y][from_x] != '#' {
            for to_y in from_y as i32 - cheat_len..=from_y as i32 + cheat_len {
                let cheat_y_len = to_y.abs_diff(from_y as i32) as i32;
                let max_cheat_x_len = cheat_len - cheat_y_len;
                for to_x in from_x as i32 - max_cheat_x_len..=from_x as i32 + max_cheat_x_len {
                    if let Some(&to_time) = time_map.get(&(to_x as usize, to_y as usize)) {
                        let cheat_cost = cheat_y_len + to_x.abs_diff(from_x as i32) as i32;
                        let time_saved = from_time as i32 - to_time as i32 - cheat_cost;
                        if time_saved >= 100 {
                            cheats_saved_100_count += 1;
                        }
                    }
                }
            }
        }
    }

    cheats_saved_100_count
}

/// Calculate the time it takes to the goal for all track positions
fn time_map(map: &[Vec<char>]) -> HashMap<(usize, usize), usize> {
    let width = map.len();
    let height = map[0].len();

    let mut end_pos = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'E' {
                end_pos = (x, y);
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut visited_map = HashMap::new();
    queue.push_back((end_pos, 0));

    while let Some((pos, time)) = queue.pop_front() {
        visited_map.insert(pos, time);

        for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neighbour_x = pos.0 as i32 + offset.0;
            let neighbour_y = pos.1 as i32 + offset.1;

            if neighbour_x >= 0 && neighbour_x < width as i32 && neighbour_y >= 0 && neighbour_y < height as i32 {
                let neighbour = (neighbour_x as usize, neighbour_y as usize);
                if !visited_map.contains_key(&neighbour) && map[neighbour.1][neighbour.0] != '#' {
                    queue.push_back((neighbour, time + 1));
                }
            }
        }
    }

    visited_map
}
