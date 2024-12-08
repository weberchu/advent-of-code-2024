use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day08.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let existing_antennas = antennas.entry(c).or_default();
                existing_antennas.push((x as i32, y as i32));
            }
        })
    }

    let mut antinodes = HashSet::new();

    for (_frequency, positions) in antennas {
        for i in 0..positions.len() {
            for j in i+1..positions.len() {
                let antinode = calculate_antinode(positions[i], positions[j], 1);
                if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                    antinodes.insert(antinode);
                }
                let antinode = calculate_antinode(positions[j], positions[i], 1);
                if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                let existing_antennas = antennas.entry(c).or_default();
                existing_antennas.push((x as i32, y as i32));
            }
        })
    }

    let mut antinodes = HashSet::new();

    for (_frequency, positions) in antennas {
        for i in 0..positions.len() {
            antinodes.insert(positions[i]);
            for j in i+1..positions.len() {
                let mut multiplier = 1;
                loop {
                    let antinode = calculate_antinode(positions[i], positions[j], multiplier);

                    if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                        antinodes.insert(antinode);
                        multiplier += 1;
                    } else {
                        break;
                    }
                }

                let mut multiplier = 1;
                loop {
                    let antinode = calculate_antinode(positions[j], positions[i], multiplier);

                    if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                        antinodes.insert(antinode);
                        multiplier += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn calculate_antinode(position1: (i32, i32), position2: (i32, i32), multiplier: i32) -> (i32, i32) {
    (position1.0 + multiplier * (position1.0 - position2.0), position1.1 + multiplier * (position1.1 - position2.1))
}
