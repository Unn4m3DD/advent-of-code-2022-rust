use eval::eval;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: String,
    test: i64,
    d1: i64,
    d2: i64,
}

impl Monkey {
    fn op(&self, value: i64) -> i64 {
        eval(&self.op.clone().replace("old", &value.to_string()))
            .unwrap()
            .as_i64()
            .unwrap()
    }
    fn test(&self, value: i64) -> i64 {
        if value % self.test == 0 {
            self.d1
        } else {
            self.d2
        }
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            lines.next();
            Monkey {
                items: lines.next().unwrap().rsplit_once(":").unwrap().1
                    .split(",")
                    .map(|e| e.trim().parse::<i64>().unwrap())
                    .collect(),
                op: lines.next().unwrap().rsplit_once("= ").unwrap().1.to_string(),
                test: lines.next().unwrap().rsplit_once("by ").unwrap().1
                    .parse::<i64>()
                    .unwrap(),
                d1: lines.next().unwrap().rsplit_once("key ").unwrap().1
                    .parse::<i64>()
                    .unwrap(),
                d2: lines.next().unwrap().rsplit_once("key ").unwrap().1
                    .parse::<i64>()
                    .unwrap(),
            }
        })
        .collect()
}

fn run_a(input: &str) {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let read_monkey = monkeys[i].clone();
            read_monkey
                .items
                .iter()
                .map(|e| read_monkey.op(*e) / 3)
                .for_each(|item| {
                    inspections[i] += 1;
                    let idx = read_monkey.test(item) as usize;
                    monkeys[i].items.remove(0);
                    monkeys[idx].items.push(item);
                })
        }
    }
    inspections.sort_unstable();
    println!(
        "a: {}",
        &inspections[inspections.len() - 2..].iter().product::<i64>()
    );
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}
fn n_lcm(list: &[i64]) -> i64 {
    list.iter().fold(1, |a, b| lcm(a, *b))
}

fn run_b(input: &str) {
    let mut monkeys = parse_input(input);
    let lcm = n_lcm(&monkeys.iter().map(|e| e.test).collect::<Vec<i64>>());
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let read_monkey = monkeys[i].clone();
            read_monkey
                .items
                .iter()
                .map(|e| read_monkey.op(*e) % lcm)
                .for_each(|item| {
                    inspections[i] += 1;
                    let idx = read_monkey.test(item) as usize;
                    monkeys[i].items.remove(0);
                    monkeys[idx].items.push(item);
                })
        }
    }
    inspections.sort_unstable();
    println!(
        "b: {:#?}",
        &inspections[inspections.len() - 2..].iter().product::<i64>()
    );
}

pub fn run() {
    let input = include_str!("../inputs/day-11.txt");
    run_a(input);
    run_b(input);
}
