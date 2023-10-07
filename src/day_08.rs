use std::collections::HashMap;

fn input_to_matrix(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<bool>>) {
    let result = input
        .split("\n")
        .map(|value| {
            value
                .chars()
                .map(|value| value.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let visibilities = input
        .split("\n")
        .map(|value| value.chars().map(|_value| false).collect())
        .collect();
    (result, visibilities)
}

fn count_visibility(
    start: (i32, i32),
    dir: (i32, i32),
    matrix: &Vec<Vec<i32>>,
    visibilities: &mut Vec<Vec<bool>>,
) {
    let mut last_height = -1;
    let mut pos = start;
    loop {
        if pos.0 < 0 || pos.0 >= matrix.len() as i32 || pos.1 < 0 || pos.1 >= matrix[0].len() as i32
        {
            return ();
        }
        if matrix[pos.0 as usize][pos.1 as usize] > last_height {
            last_height = matrix[pos.0 as usize][pos.1 as usize];
            visibilities[pos.0 as usize][pos.1 as usize] = true;
        }
        pos.0 += dir.0;
        pos.1 += dir.1;
    }
}

fn run_a(input: &str) {
    let mut dirs = HashMap::new();
    dirs.insert("right", (0, 1));
    dirs.insert("left", (0, -1));
    dirs.insert("down", (1, 0));
    dirs.insert("up", (-1, 0));

    let (matrix, mut visibilities) = input_to_matrix(input);
    for i in 0..matrix.len() {
        count_visibility(
            (i as i32, 0),
            *dirs.get("right").unwrap(),
            &matrix,
            &mut visibilities,
        );
        count_visibility(
            (i as i32, matrix[0].len() as i32 - 1),
            *dirs.get("left").unwrap(),
            &matrix,
            &mut visibilities,
        );
    }
    for i in 0..matrix[0].len() {
        count_visibility(
            (0, i as i32),
            *dirs.get("down").unwrap(),
            &matrix,
            &mut visibilities,
        );
        count_visibility(
            (matrix.len() as i32 - 1, i as i32),
            *dirs.get("up").unwrap(),
            &matrix,
            &mut visibilities,
        );
    }
    println!(
        "a: {}",
        visibilities
            .iter()
            .flat_map(|row| row.iter())
            .fold(0, |acc, next| acc + (if *next { 1 } else { 0 }))
    );
}

fn index_in_bounds(index: (i32, i32), matrix: &Vec<Vec<i32>>) -> bool {
    index.0 >= 0
        && index.0 < matrix.len() as i32
        && index.1 >= 0
        && index.1 < matrix[0].len() as i32
}

fn calculate_tree_scenicness(
    start: (i32, i32),
    matrix: &Vec<Vec<i32>>,
    dirs: &HashMap<&str, (i32, i32)>,
) -> i32 {
    let mut score = 1;
    for dir in dirs.values() {
        let mut pos = start;
        let current_tree_height = matrix[pos.0 as usize][pos.1 as usize];
        let mut dir_visibility = 0;
        loop {
            pos.0 += dir.0;
            pos.1 += dir.1;
            if !index_in_bounds(pos, matrix) {
                break;
            }
            dir_visibility += 1;
            if matrix[pos.0 as usize][pos.1 as usize] >= current_tree_height {
                break;
            }
        }
        score *= dir_visibility;
    }
    score
}

fn run_b(input: &str) {
    let mut dirs = HashMap::new();
    dirs.insert("right", (0, 1));
    dirs.insert("left", (0, -1));
    dirs.insert("down", (1, 0));
    dirs.insert("up", (-1, 0));
    let (matrix, _) = input_to_matrix(input);
    let mut max = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let score = calculate_tree_scenicness((i as i32, j as i32), &matrix, &dirs);
            if score > max {
                max = score;
            }
        }
    }

    println!("b: {}", max);
}

pub fn run() {
    let input = include_str!("../inputs/day-08.txt");
    run_a(input);
    run_b(input);
}
