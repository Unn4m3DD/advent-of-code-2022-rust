use std::collections::{BTreeSet, HashMap, HashSet};

fn move_rock(
    direction: &(i64, i64),
    rock: &mut Vec<(i64, i64)>,
    field: &mut HashSet<(i64, i64)>,
) -> bool {
    let mut can_move = true;
    for (x, y) in &*rock {
        let destination = (*x + direction.0, *y + direction.1);
        if field.contains(&destination)
            || !(0 < destination.0 && destination.0 < 8 && 0 < destination.1)
        {
            can_move = false;
            break;
        }
    }
    if can_move {
        let mut new_rock = Vec::new();
        for (x, y) in &*rock {
            new_rock.push((*x + direction.0, *y + direction.1));
        }
        *rock = new_rock;
    } else if direction.1 == -1 {
        for (x, y) in rock {
            field.insert((*x, *y));
        }
    }
    can_move
}

fn hoist_rock(rock: Vec<(i64, i64)>, highest: i64) -> Vec<(i64, i64)> {
    let mut new_rock = Vec::new();
    for (x, y) in &*rock {
        new_rock.push((*x, *y + highest + 4));
    }
    return new_rock;
}

const VALID_OPTIONS: [&str; 2] = ["<", ">"];
fn max_height(input: &str, amount_of_rocks: i64) -> i64 {
    let mut highest = 0;
    let down = (0, -1);
    let mut field = HashSet::new();
    let rock_types = vec![
        vec![(3, 0), (4, 0), (5, 0), (6, 0)],
        vec![(4, 2), (3, 1), (4, 1), (5, 1), (4, 0)],
        vec![(5, 2), (5, 1), (3, 0), (4, 0), (5, 0)],
        vec![(3, 3), (3, 2), (3, 1), (3, 0)],
        vec![(3, 1), (3, 0), (4, 1), (4, 0)],
    ];

    let mut pattern_index = 0;
    let pattern = input
        .split("")
        .filter(|e| VALID_OPTIONS.contains(e))
        .collect::<Vec<_>>();
    let mut rock_counter = 0;
    let mut current_rock = hoist_rock(rock_types[0].clone(), highest);
    let mut known_state = HashMap::new();
    let mut skipped_height = 0;
    while rock_counter < amount_of_rocks {
        let direction: (i64, i64) = match pattern[pattern_index] {
            "<" => (-1, 0),
            ">" => (1, 0),
            _ => (0, 0),
        };

        move_rock(&direction, &mut current_rock, &mut field);
        if !move_rock(&down, &mut current_rock, &mut field) {
            highest = highest.max(current_rock.first().unwrap().1);
            let last_layer = field
                .iter()
                .filter(|(_, y)| *y > highest - 300)
                .map(|&(x, y)| (x, highest - y))
                .collect::<BTreeSet<_>>();
            if let Some((previous_highest, previous_rock_counter)) = known_state.get(&(
                (rock_counter as usize % rock_types.len()),
                pattern_index,
                last_layer.clone(),
            )) {
                let height_diff = highest - *previous_highest;
                let rock_counter_diff = rock_counter - *previous_rock_counter;
                let skip_count = (amount_of_rocks - rock_counter) / rock_counter_diff;
                if skip_count > 0 {
                    skipped_height = skip_count * height_diff;
                    rock_counter += skip_count * rock_counter_diff;
                }
            } else {
                known_state.insert(
                    (
                        (rock_counter as usize % rock_types.len()),
                        pattern_index,
                        last_layer,
                    ),
                    (highest, rock_counter),
                );
            }
            rock_counter += 1;
            current_rock = hoist_rock(
                rock_types[rock_counter as usize % rock_types.len()].clone(),
                highest,
            );
        };
        pattern_index = (pattern_index + 1) % pattern.len();
    }
    highest + skipped_height
}

fn run_a(input: &str) {
    let highest = max_height(input, 2022);
    println!("a: {}", highest);
}

fn run_b(input: &str) {
    let highest = max_height(input, 1000000000000);
    println!("b: {}", highest);
}

pub fn run() {
    let input = include_str!("../inputs/day-17.txt");
    run_a(input);
    run_b(input);
}