use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("../../input/day23.txt");

    let start = Instant::now();
    let p1_ans = part1(input);
    println!("part 1: {} , took: {:?}", p1_ans, start.elapsed());

    let start = Instant::now();
    let p2_ans = part2(input);
    println!("part 2: {} , took: {:?}", p2_ans, start.elapsed());
}

fn duo<'a>(c1: &'a str, c2: &'a str) -> (&'a str, &'a str) {
    if c1.cmp(c2) == Ordering::Less {
        (c1, c2)
    } else {
        (c2, c1)
    }
}


fn trio<'a>(p0: &'a str, p1: &'a str, p2: &'a str) -> (&'a str, &'a str, &'a str) {
    let mut trio = [p0, p1, p2];
    trio.sort();
    (trio[0], trio[1], trio[2])
}

fn part1(input: &str) -> usize {
    let mut connections: HashSet<(&str, &str)> = HashSet::new();
    let mut all_computers: HashSet<&str> = HashSet::new();
    input.lines().for_each(|line| {
        let mut split = line.split("-");
        let computer_1 = split.next().unwrap();
        let computer_2 = split.next().unwrap();

        connections.insert(duo(computer_1, computer_2));

        all_computers.insert(computer_1);
        all_computers.insert(computer_2);
    });

    let mut trio_loops = HashSet::new();

    for connection in connections.iter() {
        for &c in all_computers.iter() {
            if c != connection.0 && c != connection.1 && connections.contains(&duo(connection.0, c)) && connections.contains(&duo(connection.1, c)) {
                let trio = trio(connection.0, connection.1, c);
                trio_loops.insert(trio);
            }
        }
    }

    trio_loops.iter().filter(|trio| {
        trio.0.starts_with("t") || trio.1.starts_with("t") || trio.2.starts_with("t")
    }).count()
}

fn part2(input: &str) -> String {
    let mut connections: HashSet<(&str, &str)> = HashSet::new();
    let mut all_computers: HashSet<&str> = HashSet::new();
    input.lines().for_each(|line| {
        let mut split = line.split("-");
        let computer_1 = split.next().unwrap();
        let computer_2 = split.next().unwrap();

        connections.insert(duo(computer_1, computer_2));

        all_computers.insert(computer_1);
        all_computers.insert(computer_2);
    });

    let mut largest_set = Vec::new();

    for connection in connections.iter() {
        let mut connected_set = vec![connection.0, connection.1];
        for &c in all_computers.iter() {
            if c != connection.0 && c != connection.1 && connected_set.iter().all(|connected| connections.contains(&duo(c, connected))) {
                connected_set.push(c);
            }
        }

        connected_set.sort();
        if connected_set.len() > largest_set.len() {
            largest_set = connected_set;
        }
    }

    largest_set.join(",")
}
