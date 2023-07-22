use std::cmp::Ordering;
#[derive(PartialEq, Eq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper)
            | (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}
impl From<&str> for Move {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => Move::Rock,
        }
    }
}
impl Move {
    fn value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
fn outcome(a: Move, b: Move) -> i32 {
    let mut score = match a.cmp(&b) {
        Ordering::Less => 6,
        Ordering::Equal => 3,
        Ordering::Greater => 0,
    };
    score += b.value();
    score
}

// Trying to abuse language features
fn run_a(input: &str) {
    let result = input
        .lines()
        .map(|e| {
            let operands = e.split(' ').collect::<Vec<&str>>();
            outcome(operands[0].into(), operands[1].into())
        })
        .sum::<i32>();
    println!("a: {result}")
}

// Doing the job
fn run_b(input: &str) {
    let calculate_response = |mov, outcome| ((mov + ((outcome + 2) % 3)) % 3) + 1;
    let result = input
        .lines()
        .map(|e| {
            (
                e.chars().nth(0).unwrap() as i32 - 'A' as i32,
                e.chars().nth(2).unwrap() as i32 - 'X' as i32,
            )
        })
        .map(|(mov, outcome)| outcome * 3 + calculate_response(mov, outcome))
        .sum::<i32>();
    println!("b: {result}")
}

pub fn run() {
    let input = include_str!("../inputs/day-02.txt");
    run_a(input);
    run_b(input);
}
