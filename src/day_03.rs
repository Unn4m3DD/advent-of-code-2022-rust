use itertools::Itertools;

fn calculate_priority(e: char) -> i32 {
    e as i32
        + if e.is_uppercase() {
            27 - 'A' as i32
        } else {
            1 - 'a' as i32
        }
}

fn run_a(input: &str) {
    let result = input
        .lines()
        .map(|e| (&e[..e.len() / 2], &e[e.len() / 2..]))
        .map(|(left, right)| {
            left.chars()
                .filter(|e| right.find(*e).is_some())
                .unique()
                .map(calculate_priority)
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("a: {result}")
}

fn run_b(input: &str) {
    let result = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|parts| {
            parts[0]
                .chars()
                .filter(|e| parts[1].find(*e).is_some() && parts[2].find(*e).is_some())
                .collect::<Vec<char>>()[0]
        })
        .map(calculate_priority)
        .sum::<i32>();

    println!("b: {result}")
}

pub fn run() {
    let input = include_str!("../inputs/day-03.txt");
    run_a(input);
    run_b(input);
}
