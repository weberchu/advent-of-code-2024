use std::collections::HashSet;
use std::time::Instant;
use regex::Regex;

const MAP_WIDTH: i64 = 101;
const MAP_HEIGHT: i64 = 103;
// const MAP_WIDTH: i64 = 11;
// const MAP_HEIGHT: i64 = 7;
const TIME_SECONDS: i64 = 100;

fn main() {
    let input = include_str!("../../input/day14.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> i64 {
    let mut quadrant_ne = 0;
    let mut quadrant_se = 0;
    let mut quadrant_sw = 0;
    let mut quadrant_nw = 0;

    let line_pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in input.lines() {
        let captures = line_pattern.captures(line).unwrap();
        let position_x: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let position_y: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
        let velocity_x: i64 = captures.get(3).unwrap().as_str().parse().unwrap();
        let velocity_y: i64 = captures.get(4).unwrap().as_str().parse().unwrap();

        let final_x = (position_x + velocity_x * TIME_SECONDS) % MAP_WIDTH;
        let final_x = if final_x < 0 { MAP_WIDTH + final_x } else { final_x };
        let final_y = (position_y + velocity_y * TIME_SECONDS) % MAP_HEIGHT;
        let final_y = if final_y < 0 { MAP_HEIGHT + final_y } else { final_y };

        let is_east = final_x > MAP_WIDTH / 2;
        let is_west = final_x < MAP_WIDTH / 2;
        let is_south = final_y > MAP_HEIGHT / 2;
        let is_north = final_y < MAP_HEIGHT / 2;

        if is_east {
            if is_south {
                quadrant_se += 1;
            } else if is_north {
                quadrant_ne += 1;
            }
        } else if is_west {
            if is_south {
                quadrant_sw += 1;
            } else if is_north {
                quadrant_nw += 1;
            }
        }
    }

    quadrant_ne * quadrant_se * quadrant_sw * quadrant_nw
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Robot {
    fn move_once(&mut self) {
        self.x = (self.x + self.velocity_x) % MAP_WIDTH as i32;
        if self.x < 0 {
            self.x += MAP_WIDTH as i32;
        }
        self.y = (self.y + self.velocity_y) % MAP_HEIGHT as i32;
        if self.y < 0 {
            self.y += MAP_HEIGHT as i32;
        }
    }
}

fn part2(input: &str) -> i32 {
    let line_pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = Vec::new();
    for line in input.lines() {
        let captures = line_pattern.captures(line).unwrap();
        let position_x: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let position_y: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let velocity_x: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
        let velocity_y: i32 = captures.get(4).unwrap().as_str().parse().unwrap();

        robots.push(Robot {
            x: position_x,
            y: position_y,
            velocity_x,
            velocity_y,
        });
    }

    for second in 1..=10000 {
        robots.iter_mut().for_each(|robot| robot.move_once());

        // Calculate variance of positions
        let positions: Vec<(f64, f64)> = robots.iter()
            .map(|r| (r.x as f64, r.y as f64))
            .collect();

        let mean_x = positions.iter().map(|p| p.0).sum::<f64>() / positions.len() as f64;
        let mean_y = positions.iter().map(|p| p.1).sum::<f64>() / positions.len() as f64;

        let variance = positions.iter()
            .map(|p| {
                let dx = p.0 - mean_x;
                let dy = p.1 - mean_y;
                dx * dx + dy * dy
            })
            .sum::<f64>() / positions.len() as f64;

        if variance < 1000.0 {
            println!("Position variance of second {}: {}", second, variance);
            print_robots(&robots);

            return second;
        }
    }

    -1
}

fn print_robots(robots: &[Robot]) {
    let robot_positions: HashSet<(i32, i32)> = robots.iter().map(|robot| (robot.x, robot.y)).collect();
    for y in 0..MAP_HEIGHT as i32 {
        for x in 0..MAP_WIDTH as i32 {
            if robot_positions.contains(&(x, y)) {
                print!("*");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
