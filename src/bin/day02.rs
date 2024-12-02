use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day02.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1(input: &str) -> i32 {
    let mut safe_count = 0;

    for line in input.lines() {
        let report: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();

        if is_safe(&report, false) {
            safe_count += 1;
        }
    }

    safe_count
}

fn part2(input: &str) -> i32 {
    let mut safe_count = 0;

    for line in input.lines() {
        let report: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();

        if is_safe(&report, true) {
            safe_count += 1;
        }
    }

    safe_count
}

fn is_safe(report: &[i32], can_fix_error: bool) -> bool {
    let mut is_ascending: Option<bool> = None;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff == 0 || diff.abs() > 3 {
            return can_fix_error
                && (is_safe_with_level_removed(report, 0)
                    || is_safe_with_level_removed(report, i)
                    || is_safe_with_level_removed(report, i - 1));
        }

        let current_ascending = diff > 0;
        if let Some(is_ascending) = is_ascending {
            if is_ascending != current_ascending {
                return can_fix_error
                    && (is_safe_with_level_removed(report, 0)
                        || is_safe_with_level_removed(report, i)
                        || is_safe_with_level_removed(report, i - 1));
            }
        } else {
            is_ascending = Some(current_ascending);
        }
    }

    true
}

fn is_safe_with_level_removed(report: &[i32], level_to_remove: usize) -> bool {
    let mut modified_report = report.to_owned();
    modified_report.remove(level_to_remove);
    is_safe(&modified_report, false)
}
