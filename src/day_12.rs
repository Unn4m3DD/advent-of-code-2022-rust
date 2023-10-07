use std::{collections::HashSet, vec};

use phf::phf_map;

static DIRS: phf::Map<&'static str, (i32, i32)> = phf_map! {
  "R"=> (0, 1),
  "L"=> (0, -1),
  "D"=> (1, 0),
  "U"=> (-1, 0),
};

fn parse_input(input: &str) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32), Vec<(i32, i32)>) {
    let mut start = (0, 0);
    let mut possible_starts = vec![];
    let mut end = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, e)| {
                    if e == 'S' {
                        start = (i as i32, j as i32);
                        return ('a' as u8 - 'a' as u8) as i32;
                    } else if e == 'E' {
                        end = (i as i32, j as i32);
                        return ('z' as u8 - 'a' as u8) as i32;
                    } else if e == 'a' {
                        possible_starts.push((i as i32, j as i32))
                    }
                    (e as u8 - 'a' as u8) as i32
                })
                .collect()
        })
        .collect();

    (grid, start, end, possible_starts)
}

fn calculate_min_distance(grid: &Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> i32 {
    let mut min_length = i32::MAX;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut to_visit = vec![];
    to_visit.push((start, 0));
    while to_visit.len() > 0 {
        let (current_pos, length) = to_visit.remove(0);
        if current_pos == end {
            min_length = length;
            break;
        }
        for dir in DIRS.values() {
            let new_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
            if new_pos.0 < 0
                || new_pos.1 < 0
                || new_pos.0 >= grid.len() as i32
                || new_pos.1 >= grid[0].len() as i32
            {
                continue;
            }
            if grid[new_pos.0 as usize][new_pos.1 as usize] - 1
                <= grid[current_pos.0 as usize][current_pos.1 as usize]
            {
                if !visited.contains(&new_pos) {
                    to_visit.push((new_pos, length + 1));
                    visited.insert(new_pos);
                }
            }
        }
    }
    min_length
}

fn run_a(input: &str) {
    let (grid, start, end, _) = parse_input(input);
    let min_length = calculate_min_distance(&grid, start, end);
    println!("a: {}", min_length);
}

fn run_b(input: &str) {
    let (grid, _, end, possible_starts) = parse_input(input);
    let mut min_length = i32::MAX;
    for pos in possible_starts {
        let current_length = calculate_min_distance(&grid, pos, end);
        if current_length < min_length {
            min_length = current_length;
        }
    }
    println!("b: {}", min_length);
}

pub fn run() {
    let input = include_str!("../inputs/day-12.txt");
    run_a(input);
    run_b(input);
}
