use std::env;

use advent_of_code_2022_rust::*;

static FUNCTIONS: [fn(); 3] = [day_one, day_two, day_three];

fn main() {
    let args: Vec<String> = env::args().collect();
    FUNCTIONS
        .get(args[1].parse::<usize>().unwrap() - 1)
        .unwrap()();
}
