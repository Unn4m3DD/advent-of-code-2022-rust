use std::collections::HashMap;

fn get_grid_size(input: &str) -> (i64, i64, i64, i64) {
    input.replace("\n", " -> ").split(" -> ").fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |acc, line| {
            let (y_min, y_max, x_min, x_max) = acc;
            let points = line
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let x = points[0];
            let y = points[1];
            (y_min.min(y), y_max.max(y), x_min.min(x), x_max.max(x))
        },
    )
}

fn get_grid(input: &str, x_size: i64, y_size: i64, x_offset: i64) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; x_size as usize]; y_size as usize];
    input.lines().for_each(|line| {
        let points = line
            .split(" -> ")
            .map(|point| {
                let points = point
                    .split(",")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                (points[0] - x_offset, points[1])
            })
            .collect::<Vec<(i64, i64)>>();
        points.windows(2).for_each(|points| {
            let p1 = points[0];
            let p2 = points[1];
            if p1.0 == p2.0 {
                for i in p1.1.min(p2.1)..=p2.1.max(p1.1) {
                    grid[i as usize][p1.0 as usize] = '#';
                }
            }
            if p1.1 == p2.1 {
                for i in p1.0.min(p2.0)..=p2.0.max(p1.0) {
                    grid[p1.1 as usize][i as usize] = '#';
                }
            }
        })
    });
    grid
}

fn simulate_sand(grid: &mut Vec<Vec<char>>, sand_source: (i64, i64)) -> i64 {
    let mut sand_counter = 0;
    let dirs: Vec<(i64, i64)> = vec![(1, 0), (1, -1), (1, 1)];
    'new_sand: loop {
        let mut sand_target = sand_source;
        'moving_sand: loop {
            for dir in dirs.iter() {
                let target = (sand_target.0 + dir.0, sand_target.1 + dir.1);
                if target.0 < 0
                    || target.0 >= grid.len() as i64
                    || target.1 < 0
                    || target.1 >= grid[0].len() as i64
                {
                    break 'new_sand;
                }
                if grid[target.0 as usize][target.1 as usize] == '.' {
                    sand_target = target;
                    continue 'moving_sand;
                }
            }
            break 'moving_sand;
        }
        sand_counter += 1;
        grid[sand_target.0 as usize][sand_target.1 as usize] = 'o';
    }
    sand_counter
}

fn run_a(input: &str) {
    let (_, y_max, x_min, x_max) = get_grid_size(input);
    let x_size = x_max - x_min + 1;
    let y_size = y_max + 1;
    let mut grid = get_grid(input, x_size, y_size, x_min);
    let sand_source = (0 as i64, 500 - x_min);
    let result = simulate_sand(&mut grid, sand_source);

    println!("a: {}", result);
}

fn simulate_infinite_sand(
    grid: &mut HashMap<(i64, i64), char>,
    sand_source: (i64, i64),
    y_max: i64,
) -> i64 {
    let mut sand_counter = 0;
    let dirs: Vec<(i64, i64)> = vec![(1, 0), (1, -1), (1, 1)];
    'new_sand: loop {
        let mut sand_target = sand_source;
        'moving_sand: loop {
            for dir in dirs.iter() {
                let target = (sand_target.0 + dir.0, sand_target.1 + dir.1);
                if target.0 >= y_max + 2 {
                    break 'moving_sand;
                }
                if grid.get(&target) == None {
                    sand_target = target;
                    continue 'moving_sand;
                }
            }
            break 'moving_sand;
        }
        sand_counter += 1;
        grid.insert(sand_target, 'o');
        if sand_target == sand_source {
            break 'new_sand;
        }
    }
    sand_counter
}

fn get_infinite_grid(input: &str, x_offset: i64) -> HashMap<(i64, i64), char> {
    let mut grid = HashMap::new();

    input.lines().for_each(|line| {
        let points = line
            .split(" -> ")
            .map(|point| {
                let points = point
                    .split(",")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                (points[0] - x_offset, points[1])
            })
            .collect::<Vec<(i64, i64)>>();
        points.windows(2).for_each(|points| {
            let p1 = points[0];
            let p2 = points[1];
            if p1.0 == p2.0 {
                for i in p1.1.min(p2.1)..=p2.1.max(p1.1) {
                    grid.insert((i, p1.0), '#');
                }
            }
            if p1.1 == p2.1 {
                for i in p1.0.min(p2.0)..=p2.0.max(p1.0) {
                    grid.insert((p1.1, i), '#');
                }
            }
        })
    });
    grid
}

fn run_b(input: &str) {
    let (_, y_max, x_min, _) = get_grid_size(input);
    let mut grid: HashMap<(i64, i64), char> = get_infinite_grid(input, x_min);
    let sand_source = (0 as i64, 500 - x_min);
    let result = simulate_infinite_sand(&mut grid, sand_source, y_max);
    println!("b: {}", result);
}

pub fn run() {
    let input = include_str!("../inputs/day-14.txt");
    run_a(input);
    run_b(input);
}
