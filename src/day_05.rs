fn transfer(stacks: &mut Vec<Vec<char>>, from: usize, to: usize) {
    let to_move = stacks[from].pop().unwrap();
    stacks[to].push(to_move);
}
fn transfer_n(stacks: &mut Vec<Vec<char>>, from: usize, to: usize, n: usize) {
    let mut to_move = vec![];
    for _ in 0..n {
        to_move.push(stacks[from].pop().unwrap());
    }
    for _ in 0..n {
        stacks[to].push(to_move.pop().unwrap());
    }
}

fn run_a(input: &str) {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let [initial_state, moves] = [
        input[0].split('\n').collect::<Vec<&str>>(),
        input[1].lines().collect::<Vec<&str>>(),
    ];
    let stack_count = initial_state[0].len() / 4;
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..=stack_count {
        stacks.push(vec![])
    }
    for line in initial_state.iter() {
        for (idx, item) in line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|e| e[1])
            .collect::<Vec<char>>()
            .iter()
            .enumerate()
        {
            if *item != ' ' && !item.is_numeric() {
                stacks[idx].insert(0, *item)
            }
        }
    }
    moves
        .iter()
        .map(|e| e.split(' ').collect::<Vec<&str>>())
        .map(|e| {
            (
                e[1].parse::<usize>().unwrap(),
                e[3].parse::<usize>().unwrap() - 1,
                e[5].parse::<usize>().unwrap() - 1,
            )
        })
        .for_each(|(count, from, to)| {
            for _ in 0..count {
                transfer(&mut stacks, from, to)
            }
        });
    let result = stacks.iter().map(|e| e[e.len() - 1]).collect::<String>();
    println!("a: {}", result);
}

fn run_b(input: &str) {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let [initial_state, moves] = [
        input[0].split('\n').collect::<Vec<&str>>(),
        input[1].lines().collect::<Vec<&str>>(),
    ];
    let stack_count = initial_state[0].len() / 4;
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..=stack_count {
        stacks.push(vec![])
    }
    for line in initial_state.iter() {
        for (idx, item) in line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|e| e[1])
            .collect::<Vec<char>>()
            .iter()
            .enumerate()
        {
            if *item != ' ' && !item.is_numeric() {
                stacks[idx].insert(0, *item)
            }
        }
    }
    moves
        .iter()
        .map(|e| e.split(' ').collect::<Vec<&str>>())
        .map(|e| {
            (
                e[1].parse::<usize>().unwrap(),
                e[3].parse::<usize>().unwrap() - 1,
                e[5].parse::<usize>().unwrap() - 1,
            )
        })
        .for_each(|(count, from, to)| {
            transfer_n(&mut stacks, from, to, count);
        });
    let result = stacks.iter().map(|e| e[e.len() - 1]).collect::<String>();
    println!("b: {}", result);
}

pub fn run() {
    let input = include_str!("../inputs/day-05.txt");
    run_a(input);
    run_b(input);
}
