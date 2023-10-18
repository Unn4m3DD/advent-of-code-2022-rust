use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
};

use fn_cache::{FnCache, HashCache};

fn parse_input(input: &str) -> HashMap<&str, (Vec<&str>, i64)> {
    let mut graph = HashMap::new();
    input.lines().for_each(|line| {
        let node = line
            .split_once(" has ")
            .unwrap()
            .0
            .split_once("Valve ")
            .unwrap()
            .1;
        let rate = line
            .split_once(" rate=")
            .unwrap()
            .1
            .split_once(";")
            .unwrap()
            .0
            .parse::<i64>()
            .unwrap();

        let connections = line
            .split_once(" to valve")
            .unwrap()
            .1
            .split_once(" ")
            .unwrap()
            .1
            .split(", ")
            .collect();
        graph.insert(node, (connections, rate));
    });
    graph
}

fn run_a(input: &str) {
    let graph = parse_input(input);
    let mut cached_explore = HashCache::<(&str, i64, Rc<BTreeSet<&str>>, i64), i64>::recursive(
        |cache, (current_node, time_remaining, released, extra_player)| {
            let (connections, rate) = graph.get(current_node).unwrap();
            let mut result = 0;
            if *time_remaining == 0 || *time_remaining == -1 {
                if *extra_player == 0 {
                    return 0;
                } else {
                    return *cache.get(("AA", 25, (*released).clone(), extra_player - 1));
                }
            }
            if !released.contains(*current_node) && *rate != 0 {
                let mut new_released = BTreeSet::clone(released);
                new_released.insert(*current_node);
                let new_released = Rc::from(new_released);
                for connection in connections {
                    let attempt = *cache.get((
                        connection,
                        time_remaining - 2,
                        new_released.clone(),
                        *extra_player,
                    ));
                    result = ((attempt) + time_remaining * rate).max(result);
                }
            }
            for connection in connections {
                result = (*cache.get((
                    connection,
                    time_remaining - 1,
                    (*released).clone(),
                    *extra_player,
                )))
                .max(result);
            }
            result
        },
    );
    let released = Rc::new(BTreeSet::new());
    let result = cached_explore.get(("AA", 29, released, 0));
    println!("a: {}", result);
}

fn run_b(input: &str) {
    let graph = parse_input(input);
    let mut cached_explore = HashCache::<(&str, i64, Rc<BTreeSet<&str>>, i64), i64>::recursive(
        |cache, (current_node, time_remaining, released, extra_player)| {
            let (connections, rate) = graph.get(current_node).unwrap();
            let mut result = 0;
            if released.len() == graph.len() {
                return 0;
            }
            if *time_remaining == 0 || *time_remaining == -1 {
                if *extra_player == 0 {
                    return 0;
                } else {
                    return *cache.get(("AA", 25, (*released).clone(), extra_player - 1));
                }
            }
            if !released.contains(*current_node) && *rate != 0 {
                let mut new_released = BTreeSet::clone(released);
                new_released.insert(*current_node);
                let new_released = Rc::from(new_released);
                for connection in connections {
                    let attempt = *cache.get((
                        connection,
                        time_remaining - 2,
                        new_released.clone(),
                        *extra_player,
                    ));
                    result = ((attempt) + time_remaining * rate).max(result);
                }
            }
            for connection in connections {
                result = (*cache.get((
                    connection,
                    time_remaining - 1,
                    (*released).clone(),
                    *extra_player,
                )))
                .max(result);
            }
            result
        },
    );
    let released = Rc::new(BTreeSet::new());
    let result = cached_explore.get(("AA", 25, released, 1));
    println!("b: {}", result);
}

pub fn run() {
    let input = include_str!("../inputs/day-16.txt");
    run_a(input);
    run_b(input);
}
