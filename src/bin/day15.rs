use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day15.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves = "".to_string();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        map.push(line.chars().collect());
    }

    for line in lines {
        moves.push_str(line);
    }

    let mut robot = find_robot(&map);

    for c in moves.chars() {
        let offset: (i32, i32) = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid move"),
        };

        let mut next = (robot.0 + offset.0, robot.1 + offset.1);
        let mut boxes_to_move = 0;

        while map[next.1 as usize][next.0 as usize] == 'O' {
            boxes_to_move += 1;
            next = (next.0 + offset.0, next.1 + offset.1);
        }

        if map[next.1 as usize][next.0 as usize] == '.' {
            for _ in 0..boxes_to_move {
                map[next.1 as usize][next.0 as usize] = 'O';
                next = (next.0 - offset.0, next.1 - offset.1);
            }

            map[next.1 as usize][next.0 as usize] = '@';
            robot = (next.0, next.1);
            next = (next.0 - offset.0, next.1 - offset.1);
            map[next.1 as usize][next.0 as usize] = '.';
        }
    }

    gps_sum(&map, 'O')
}

fn find_robot(map: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '@' {
                return (x as i32, y as i32);
            }
        }
    }

    panic!("No robot found");
}

fn gps_sum(map: &[Vec<char>], target: char) -> usize {
    let mut gps_sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == target {
                gps_sum += 100 * y + x;
            }
        }
    }

    gps_sum
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves = "".to_string();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        map.push(
            line
                .chars()
                .flat_map(|c| {
                    match c {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '.' => ['.', '.'],
                        '@' => ['@', '.'],
                        _ => panic!("unexpected map entry {}", c),
                    }
                })
                .collect()
        );
    }

    for line in lines {
        moves.push_str(line);
    }

    let mut robot = find_robot(&map);

    for c in moves.chars() {
        let offset: (i32, i32) = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid move"),
        };

        let mut can_move = false;
        let mut pos_to_move: Vec<HashSet<(i32, i32)>> = vec![HashSet::from([robot])];

        loop {
            let next_check: HashSet<(i32, i32)> = pos_to_move
                .last()
                .unwrap()
                .iter()
                .map(|p| (p.0 + offset.0, p.1 + offset.1))
                .collect();

            let is_blocked = next_check.iter().any(|pos| map[pos.1 as usize][pos.0 as usize] == '#');
            if is_blocked {
                break;
            }

            let can_next_move = next_check.iter().all(|pos| map[pos.1 as usize][pos.0 as usize] == '.');
            if can_next_move {
                can_move = true;
                break;
            }

            let mut boxes_to_move = HashSet::new();
            next_check.iter().for_each(|pos| {
                if offset.0 == 0 {
                    match map[pos.1 as usize][pos.0 as usize] {
                        '[' => {
                            boxes_to_move.insert(*pos);
                            boxes_to_move.insert((pos.0 + 1, pos.1));
                        },
                        ']' => {
                            boxes_to_move.insert(*pos);
                            boxes_to_move.insert((pos.0 - 1, pos.1));
                        },
                        _ => {},
                    }
                } else {
                    match map[pos.1 as usize][pos.0 as usize] {
                        '[' | ']' => {
                            boxes_to_move.insert(*pos);
                        },
                        _ => {},
                    }
                }
            });
            pos_to_move.push(boxes_to_move);
        }

        if can_move {
            for pos in pos_to_move.iter().rev() {
                for p in pos.iter() {
                    map[(p.1 + offset.1) as usize][(p.0 + offset.0) as usize] = map[p.1 as usize][p.0 as usize];
                    map[p.1 as usize][p.0 as usize] = '.';
                }
            }

            robot = (robot.0 + offset.0, robot.1 + offset.1);
        }
    }

    gps_sum(&map, '[')
}
