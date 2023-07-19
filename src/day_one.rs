pub fn run() {
    let input = include_str!("../inputs/day-one.txt");
    let mut result = input
        .split("\n\n")
        .map(|e| e.lines().map(|e| e.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();
    result.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    let sum = result.get(0..3).unwrap().iter().sum::<i32>();
    println!("a: {}", result[0]);
    println!("b: {}", sum);
}
