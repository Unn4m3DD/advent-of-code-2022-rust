use std::collections::HashSet;

fn run_a(input: &str) {
    for (index, item) in input.as_bytes().windows(4).enumerate() {
        let mut uniq = HashSet::new();

        if item.into_iter().all(move |x| uniq.insert(x)) {
            println!("a: {}", index + 4);
            return;
        }
    }
}

fn run_b(input: &str) {
    for (index, item) in input.as_bytes().windows(14).enumerate() {
        let mut uniq = HashSet::new();

        if item.into_iter().all(move |x| uniq.insert(x)) {
            println!("b: {}", index + 14);
            return;
        }
    }
}

pub fn run() {
    let input = include_str!("../inputs/day-06.txt");
    run_a(input);
    run_b(input);
}
