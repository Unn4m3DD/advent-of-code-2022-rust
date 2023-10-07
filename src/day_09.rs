use std::collections::HashSet;

use phf::phf_map;

static DIRS: phf::Map<&'static str, (i32, i32)> = phf_map! {
  "R"=> (0, 1),
  "L"=> (0, -1),
  "D"=> (1, 0),
  "U"=> (-1, 0),
};

fn calculate_tail_pos(tail_pos: &(i32, i32), head_pos: &(i32, i32)) -> (i32, i32) {
    if head_pos.0 - 1 <= tail_pos.0
        && tail_pos.0 <= head_pos.0 + 1
        && head_pos.1 - 1 <= tail_pos.1
        && tail_pos.1 <= head_pos.1 + 1
    {
        return tail_pos.clone();
    }
    if tail_pos.0 == head_pos.0 {
        return (tail_pos.0, tail_pos.1 + (head_pos.1 - tail_pos.1).signum());
    }
    if tail_pos.1 == head_pos.1 {
        return (tail_pos.0 + (head_pos.0 - tail_pos.0).signum(), tail_pos.1);
    }
    return (
        tail_pos.0 + (head_pos.0 - tail_pos.0).signum(),
        tail_pos.1 + (head_pos.1 - tail_pos.1).signum(),
    );
}

fn run_a(input: &str) {
    let mut visited = HashSet::new();
    let mut tail_pos = (0, 0);
    let mut head_pos = (0, 0);
    for line in input.lines() {
        let mut commands = line.split(" ");
        let dir = DIRS.get(commands.next().unwrap()).unwrap();
        let distance = commands.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..distance {
            head_pos.0 += dir.0;
            head_pos.1 += dir.1;
            tail_pos = calculate_tail_pos(&tail_pos, &head_pos);
            visited.insert(tail_pos);
        }
    }
    println!("a: {:?}", visited.len());
}

fn run_b(input: &str) {
    let mut visited = HashSet::new();
    let mut body = [(0, 0)].repeat(10);

    for line in input.lines() {
        let mut commands = line.split(" ");
        let dir = DIRS.get(commands.next().unwrap()).unwrap();
        let distance = commands.next().unwrap().parse::<i32>().unwrap();
        for _ in 0..distance {
            body[0].0 += dir.0;
            body[0].1 += dir.1;
            for i in 1..body.len() {
                body[i] = calculate_tail_pos(&body[i], &body[i - 1]);
            }
            visited.insert(body[9].clone());
        }
    }
    println!("a: {:?}", visited.len());
}

pub fn run() {
    let input = include_str!("../inputs/day-09.txt");
    run_a(input);
    run_b(input);
}
