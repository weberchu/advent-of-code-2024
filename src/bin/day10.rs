use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day10.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<i32>> = input.lines().map(|line| line.chars().map(|c| c as i32 - '0' as i32).collect()).collect();
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut score = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] == 0 {
                score += find_trails_1(&map, height, width, x, y).len();
            }
        }
    }

    score
}

fn find_trails_1(map: &Vec<Vec<i32>>, height: i32, width: i32, x: i32, y: i32) -> HashSet<(i32, i32)> {
    let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut trails_found = HashSet::new();
    let next_height = map[y as usize][x as usize] + 1;

    for offset in offsets {
        let neighbour_x = x + offset.0;
        let neighbour_y = y + offset.1;

        if neighbour_x >= 0 && neighbour_x < width && neighbour_y >= 0 && neighbour_y < height
            && map[neighbour_y as usize][neighbour_x as usize] == next_height {
            if next_height == 9 {
                trails_found.insert((neighbour_x, neighbour_y));
            } else {
                trails_found.extend(find_trails_1(map, height, width, neighbour_x, neighbour_y));
            }
        }
    }

    trails_found
}

fn part2(input: &str) -> i32 {
    let map: Vec<Vec<i32>> = input.lines().map(|line| line.chars().map(|c| c as i32 - '0' as i32).collect()).collect();
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut score = 0;

    for y in 0..height {
        for x in 0..width {
            if map[y as usize][x as usize] == 0 {
                score += find_trails_2(&map, height, width, x, y);
            }
        }
    }

    score
}

fn find_trails_2(map: &Vec<Vec<i32>>, height: i32, width: i32, x: i32, y: i32) -> i32{
    let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut trails_found = 0;
    let next_height = map[y as usize][x as usize] + 1;

    for offset in offsets {
        let neighbour_x = x + offset.0;
        let neighbour_y = y + offset.1;

        if neighbour_x >= 0 && neighbour_x < width && neighbour_y >= 0 && neighbour_y < height
            && map[neighbour_y as usize][neighbour_x as usize] == next_height {
            if next_height == 9 {
                trails_found += 1;
            } else {
                trails_found += find_trails_2(map, height, width, neighbour_x, neighbour_y);
            }
        }
    }

    trails_found
}
