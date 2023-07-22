use std::env;

use advent_of_code_2022_rust::*;

static FUNCTIONS: [fn(); 5] = [day_01, day_02, day_03, day_04, day_05];

fn main() {
    let args: Vec<String> = env::args().collect();
    FUNCTIONS
        .get(args[1].parse::<usize>().unwrap() - 1)
        .unwrap()();
}
