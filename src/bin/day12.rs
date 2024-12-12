use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day12.txt");

    let start = Instant::now();
    let (p1_ans, p2_ans) = part1_and_2(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
enum Border {
    N, E, S, W
}

impl Border {
    fn offset(&self) -> (i32, i32) {
        match self {
            Border::N => (0, -1),
            Border::E => (1, 0),
            Border::S => (0, 1),
            Border::W => (-1, 0),
        }
    }

    fn adjacent_offsets(&self) -> [(i32, i32); 2] {
        match self {
            Border::N | Border::S => [(1, 0), (-1, 0)],
            Border::E | Border::W => [(0, 1), (0, -1)],
        }
    }
}

const ALL_BORDERS: [Border; 4] = [Border::N, Border::E, Border::S, Border::W];

fn part1_and_2(input: &str) -> (i32, i32) {
    let map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut total_price_1 = 0;
    let mut total_price_2 = 0;
    let mut visited = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            if !visited.contains(&(x, y)) {
                let mut area = 0;
                let mut borders = HashSet::new();
                let plot_type = map[y as usize][x as usize];
                let mut plots_to_check = vec![(x, y)];
                while !plots_to_check.is_empty() {
                    let plot = plots_to_check.remove(0);
                    area += 1;
                    for b in ALL_BORDERS {
                        let neighbour_x = plot.0 + b.offset().0;
                        let neighbour_y = plot.1 + b.offset().1;
                        if neighbour_x >= 0 && neighbour_x < width && neighbour_y >= 0 && neighbour_y < height && map[neighbour_y as usize][neighbour_x as usize] == plot_type {
                            if !plots_to_check.contains(&(neighbour_x, neighbour_y)) && !visited.contains(&(neighbour_x, neighbour_y)) {
                                plots_to_check.push((neighbour_x, neighbour_y));
                            }
                        } else {
                            borders.insert((plot, b));
                        }
                    }

                    visited.insert(plot);
                }

                total_price_1 += area * borders.len() as i32;
                total_price_2 += area * count_sides(&mut borders);
            }
        }
    }

    (total_price_1, total_price_2)
}

fn count_sides(borders: &mut HashSet<((i32, i32), Border)>) -> i32 {
    let mut side_count = 0;

    while !borders.is_empty() {
        let &(plot, border) = borders.iter().take(1).next().unwrap();
        borders.remove(&(plot, border));
        for adjacent_offset in border.adjacent_offsets() {
            let mut neighbour = (plot.0 + adjacent_offset.0, plot.1 + adjacent_offset.1);
            while borders.contains(&(neighbour, border)) {
                borders.remove(&(neighbour, border));
                neighbour = (neighbour.0 + adjacent_offset.0, neighbour.1 + adjacent_offset.1);
            }
        }

        side_count += 1;
    }

    side_count
}
