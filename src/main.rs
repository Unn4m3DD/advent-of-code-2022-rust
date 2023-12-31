use std::env;

use advent_of_code_2022_rust::*;

fn main() {
    let functions = [
        day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11,
        day_12, day_13, day_14, day_15, day_16, day_17, day_18,
    ];
    let args: Vec<String> = env::args().collect();
    functions
        .get(args[1].parse::<usize>().unwrap() - 1)
        .unwrap()();
}
