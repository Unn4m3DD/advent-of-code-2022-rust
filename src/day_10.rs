use itertools::Itertools;

fn run_a(input: &str) {
    let breakpoints = [20, 60, 100, 140, 180, 220];
    let mut current_cycle = 0;
    let mut signal_strength = 0;
    let mut value_of_x = 1;
    input.lines().for_each(|line| {
        current_cycle += 1;
        if breakpoints.contains(&current_cycle) {
            signal_strength += value_of_x * current_cycle;
        }
        if line != "noop" {
            current_cycle += 1;
            if breakpoints.contains(&current_cycle) {
                signal_strength += value_of_x * current_cycle;
            }
            let value = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            value_of_x += value;
        }
    });
    println!("a: {:?}", signal_strength);
}

fn run_b(input: &str) {
    let mut screen = vec![vec![false; 40]; 6];
    let mut current_cycle = 0;
    let mut current_line = 0;
    let mut value_of_x: i32 = 1;
    input.lines().for_each(|line| {
        if current_cycle - 1 <= value_of_x && value_of_x <= current_cycle + 1 {
            screen[current_line][current_cycle as usize] = true;
        }
        current_cycle += 1;
        if current_cycle == 40 {
            current_cycle = 0;
            current_line += 1;
        }
        if line != "noop" {
            if current_cycle - 1 <= value_of_x && value_of_x <= current_cycle + 1 {
                screen[current_line][current_cycle as usize] = true;
            }
            current_cycle += 1;
            if current_cycle == 40 {
                current_cycle = 0;
                current_line += 1;
            }
            let value = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            value_of_x += value;
        }
    });
    println!(
        "b:\n{}",
        screen
            .iter()
            .map(|row| row.iter().map(|e| if *e { "█" } else { " " }).join(""))
            .join("\n")
    );
}

pub fn run() {
    let input = include_str!("../inputs/day-10.txt");
    run_a(input);
    run_b(input);
}
