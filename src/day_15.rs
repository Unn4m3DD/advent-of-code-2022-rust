use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
    range: i64,
}

impl Sensor {
    fn is_in_range(&self, pos: (i64, i64)) -> bool {
        manhattan_distance(self.pos, pos) <= self.range
    }
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_input(input: &str) -> (Vec<Sensor>, i64, i64) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    (
        input
            .replace("Sensor at ", "")
            .replace(": closest beacon is at ", ";")
            .replace("x=", "")
            .replace(" y=", "")
            .lines()
            .map(|line| {
                let points = line
                    .split(";")
                    .map(|point| {
                        let (x, y) = point.split_once(",").unwrap();
                        return (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
                    })
                    .collect::<Vec<_>>();
                let range = manhattan_distance(points[0], points[1]);
                min_x = min_x.min(points[0].0 - range + 1);
                min_x = min_x.min(points[1].0 - range + 1);
                max_x = max_x.max(points[0].1 + range - 1);
                max_x = max_x.max(points[1].1 + range - 1);
                Sensor {
                    pos: points[0],
                    beacon: points[1],
                    range: manhattan_distance(points[0], points[1]),
                }
            })
            .collect(),
        min_x,
        max_x,
    )
}

fn run_a(input: &str) {
    let (sensors, min_x, max_x) = parse_input(input);
    let mut beacon_positions = HashSet::new();
    sensors.iter().map(|e| e.beacon).for_each(|b| {
        beacon_positions.insert(b);
    });
    let result = (min_x..=max_x)
        .filter(|&i| {
            sensors.iter().fold(false, |acc, next| {
                acc || (next.is_in_range((i, 2000000)) && !beacon_positions.contains(&(i, 2000000)))
            })
        })
        .count();
    println!("a: {}", result);
}

fn run_b(input: &str) {
    let (sensors, _, _) = parse_input(input);

    let mut candidates = sensors
        .iter()
        .map(|sensor| {
            let min_x = sensor.pos.0 - sensor.range - 1;
            let max_x = sensor.pos.0 + sensor.range + 1;
            (min_x..=max_x)
                .map(|x| {
                    if x <= sensor.pos.0 {
                        vec![
                            (x, sensor.pos.1 + (x - min_x)),
                            (x, sensor.pos.1 - (x - min_x)),
                        ]
                    } else {
                        vec![
                            (x, sensor.pos.1 + (sensor.range + 1 - (x - sensor.pos.0))),
                            (x, sensor.pos.1 - (sensor.range + 1 - (x - sensor.pos.0))),
                        ]
                    }
                })
                .flat_map(|e| e)
                .collect::<Vec<_>>()
        })
        .flat_map(|e| e)
        .filter(|e| e.1 >= 0 && e.0 >= 0 && e.1 <= 4000000 && e.0 <= 4000000);
    let result = candidates.find(|(x, y)| {
        sensors
            .iter()
            .fold(true, |acc, next| acc && (!next.is_in_range((*x, *y))))
    });
    println!("b: {:?}", result.unwrap().0 * 4000000 + result.unwrap().1);
}

pub fn run() {
    let input = include_str!("../inputs/day-15.txt");
    run_a(input);
    run_b(input);
}
