use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day25.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    while let Some(first_line) = lines.next() {
        let row_1: Vec<char> = lines.next().unwrap().chars().collect();
        let row_2: Vec<char> = lines.next().unwrap().chars().collect();
        let row_3: Vec<char> = lines.next().unwrap().chars().collect();
        let row_4: Vec<char> = lines.next().unwrap().chars().collect();
        let row_5: Vec<char> = lines.next().unwrap().chars().collect();

        let pin_heights: Vec<usize> = (0..5).map(|pin| {
            let pins = [
                row_1[pin],
                row_2[pin],
                row_3[pin],
                row_4[pin],
                row_5[pin],
            ];
            pins.iter().filter(|&&pin| pin == '#').count()
        }).collect();

        match first_line {
            "#####" => {
                locks.push(pin_heights);
            },
            "....." => {
                keys.push(pin_heights);
            },
            _ => panic!("Invalid line {}", first_line),
        };

        lines.next();
        lines.next();
    }

    let mut fit_count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if (0..5).map(|pin| {
                lock[pin] + key[pin]
            }).all(|total_height| total_height <= 5) {
                fit_count += 1;
            }
        }
    }

    fit_count
}

fn part2(input: &str) -> usize {
    input.len()
}
