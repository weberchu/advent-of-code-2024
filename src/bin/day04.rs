use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day04.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let width = input[0].len();
    let height = input.len();

    let mut total = 0;

    for i in 0..width {
        for j in 0..height {
            if input[j][i] == 'X' {
                let count: usize = offsets_1().iter()
                    .map(|offset| {
                        look_for(&['M', 'A', 'S'], &input, width, height, i, j, offset)
                    })
                    .sum();
                total += count;
            }
        }
    }

    total
}

fn look_for(s: &[char], input: &Vec<Vec<char>>, width: usize, height: usize, x: usize, y: usize, offset: &(i32, i32)) -> usize {
    let x = x as i32 + offset.0;
    let y = y as i32 + offset.1;

    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
        let x = x as usize;
        let y = y as usize;
        if input[y][x] == s[0] {
            return if s.len() == 1 {
                1
            } else {
                look_for(&s[1..], input, width, height, x, y, offset)
            }
        }
    }

    0
}

fn offsets_1() -> Vec<(i32, i32)> {
    vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ]
}

fn part2(input: &str) -> usize {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let width = input[0].len();
    let height = input.len();

    let mut total = 0;

    let look_for_corners = ['M', 'M', 'S', 'S'];

    for i in 1..width - 1 {
        for j in 1..height - 1 {
            if input[j][i] == 'A' {
                for offsets in offsets_2() {
                    let mut is_match = true;
                    for (k, offset) in offsets.iter().enumerate() {
                        let x = (i as i32 + offset.0) as usize;
                        let y = (j as i32 + offset.1) as usize;

                        if input[y][x] != look_for_corners[k] {
                            is_match = false;
                            break;
                        }
                    }

                    if is_match {
                        total += 1;
                        break;
                    }
                }
            }
        }
    }

    total
}

fn offsets_2() -> Vec<Vec<(i32, i32)>> {
    vec![
        vec![(-1, -1), (1, -1), (1, 1), (-1, 1)],
        vec![(1, -1), (1, 1), (-1, 1), (-1, -1)],
        vec![(1, 1), (-1, 1), (-1, -1), (1, -1)],
        vec![(-1, 1), (-1, -1), (1, -1), (1, 1)],
    ]
}