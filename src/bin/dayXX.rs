use std::time::Instant;

fn main() {
    let input = include_str!("../../input/example.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> usize {
    input.len()
}

fn part2(input: &str) -> usize {
    input.len()
}
