use std::collections::HashSet;

fn parse_vec_2(input: &str) -> (i64, i64, i64) {
    let mut iter = input.split(",");
    let x = iter.next().unwrap().parse::<i64>().unwrap() * 2;
    let y = iter.next().unwrap().parse::<i64>().unwrap() * 2;
    let z = iter.next().unwrap().parse::<i64>().unwrap() * 2;
    (x, y, z)
}
fn parse_vec(input: &str) -> (i64, i64, i64) {
    let mut iter = input.split(",");
    let x = iter.next().unwrap().parse::<i64>().unwrap();
    let y = iter.next().unwrap().parse::<i64>().unwrap();
    let z = iter.next().unwrap().parse::<i64>().unwrap();
    (x, y, z)
}

fn run_a(input: &str) {
    let faces = [
        [0, 1, 1],
        [1, 0, 1],
        [1, 1, 0],
        [2, 1, 1],
        [1, 2, 1],
        [1, 1, 2],
    ];
    let result = input
        .lines()
        .flat_map(|line| {
            let (x, y, z) = parse_vec_2(line);
            faces.clone().map(|e| (x + e[0], y + e[1], z + e[2]))
        })
        .collect::<Vec<_>>();
    let total_faces = result.len();
    let overlapping_faces = total_faces - result.iter().collect::<HashSet<_>>().len();
    let result = total_faces - overlapping_faces * 2;
    println!("a: {}", result);
}

fn run_b(input: &str) {
    let directions = [
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];
    let cubes = input.lines().map(parse_vec).collect::<HashSet<_>>();
    let max = cubes.iter().fold((0, 0, 0), |(a, b, c), (x, y, z)| {
        (a.max(*x), b.max(*y), c.max(*z))
    });
    let mut to_explore = vec![(0, 0, 0)];
    let mut explored = HashSet::new();
    let mut surface_count = 0;
    while let Some(current_node) = to_explore.pop() {
        if explored.contains(&current_node) {
            continue;
        }
        explored.insert(current_node);
        let neighbors = directions
            .iter()
            .map(|(x, y, z)| (current_node.0 + x, current_node.1 + y, current_node.2 + z))
            .filter(|e| {
                -1 <= e.0
                    && e.0 <= max.0 + 1
                    && -1 <= e.1
                    && e.1 <= max.1 + 1
                    && -1 <= e.2
                    && e.2 <= max.2 + 1
            });
        neighbors.for_each(|e| {
            if cubes.contains(&e) {
                surface_count += 1;
            } else {
                to_explore.push(e);
            }
        })
    }
    println!("b: {}", surface_count);
}

pub fn run() {
    let input = include_str!("../inputs/day-18.txt");
    run_a(input);
    run_b(input);
}
