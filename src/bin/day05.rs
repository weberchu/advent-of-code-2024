use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day05.txt");

    let start = Instant::now();
    let (p1_ans, p2_ans) = part1and2(input);
    println!("part 1: {:?} , took: {:?}", p1_ans, start.elapsed());
    println!("part 2: {:?} , took: {:?}", p2_ans, start.elapsed());
}

fn part1and2(input: &str) -> (i32, i32) {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut dependencies = HashSet::new();

    while !lines[i].is_empty() {
        dependencies.insert(lines[i]);
        i += 1;
    }

    // skip empty line
    i += 1;

    let mut valid_total = 0;
    let mut invalid_total = 0;

    for line in lines[i..].iter() {
        let pages: Vec<i32> = line.split(",").map(|n| n.parse().unwrap()).collect();

        let mut is_valid = true;
        for (j, page1) in pages.iter().enumerate() {
            for page2 in pages[(j+1)..].iter() {
                let invalid_lookup = format!("{}|{}", page2, page1);
                if dependencies.contains(invalid_lookup.as_str()) {
                    is_valid = false;

                    let fixed_pages = fix_pages(&pages, &dependencies);
                    invalid_total += fixed_pages[fixed_pages.len() / 2];

                    break;
                }
            }

            if !is_valid {
                break;
            }
        }
        if is_valid {
            valid_total += pages[pages.len() / 2];
        }
    }

    (valid_total, invalid_total)
}

fn fix_pages(pages: &Vec<i32>, dependencies: &HashSet<&str>) -> Vec<i32> {
    for (j, page1) in pages.iter().enumerate() {
        for (k, page2) in pages[(j+1)..].iter().enumerate() {
            let invalid_lookup = format!("{}|{}", page2, page1);
            if dependencies.contains(invalid_lookup.as_str()) {
                let mut pages = pages.to_owned();
                pages.swap(j, j + k + 1);
                return fix_pages(&pages, dependencies);
            }
        }
    }

    pages.to_owned()
}
