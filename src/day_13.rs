use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Packet {
    Value(i64),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(a), Packet::Value(b)) => a.cmp(b),
            (Packet::Value(a), Packet::List(b)) => {
                Packet::List(vec![Packet::Value(*a)]).cmp(&Packet::List(b.clone()))
            }
            (Packet::List(a), Packet::Value(b)) => {
                Packet::List(a.clone()).cmp(&Packet::List(vec![Packet::Value(*b)]))
            }
            (Packet::List(a), Packet::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    let comparison = a.cmp(b);
                    if comparison != Ordering::Equal {
                        return comparison;
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

fn run_a(input: &str) {
    let result: i64 = input
        .split("\n\n")
        .enumerate()
        .map(|(idx, lines)| {
            let lines = lines.lines().collect::<Vec<&str>>();
            let line1: Packet = serde_json::from_str(lines[0]).unwrap();
            let line2: Packet = serde_json::from_str(lines[1]).unwrap();

            return if matches!(line1.cmp(&line2), Ordering::Less) {
                (idx + 1) as i64
            } else {
                0
            };
        })
        .sum();
    println!("a: {}", result);
}

fn run_b(input: &str) {
    let sep1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let sep2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
    let mut result = input
        .replace("\n\n", "\n")
        .lines()
        .map(|line| serde_json::from_str::<Packet>(line).unwrap())
        .collect::<Vec<Packet>>();

    result.push(sep1.clone());
    result.push(sep2.clone());
    result.sort();
    let idx1 = result
        .iter()
        .enumerate()
        .find(|(_, &ref value)| *value == sep1)
        .unwrap()
        .0
        + 1;
    let idx2 = result
        .iter()
        .enumerate()
        .find(|(_, &ref value)| *value == sep2)
        .unwrap()
        .0
        + 1;
    println!("b: {}", idx1 * idx2);
}

pub fn run() {
    let input = include_str!("../inputs/day-13.txt");
    run_a(input);
    run_b(input);
}
