fn run_a(input: &str) {
    let result = input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|pair| {
            (
                pair[0]
                    .split('-')
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
                pair[1]
                    .split('-')
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .map(|line| ((line.0[0], line.0[1]), (line.1[0], line.1[1])))
        .filter(|((a, b), (c, d))| (a <= c && b >= d) || (a >= c && b <= d))
        .count();
    println!("a: {result:?}")
}

fn run_b(input: &str) {
    let result = input
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|pair| {
            (
                pair[0]
                    .split('-')
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
                pair[1]
                    .split('-')
                    .map(|e| e.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .map(|line| ((line.0[0], line.0[1]), (line.1[0], line.1[1])))
        .filter(|((a, b), (c, d))| a <= d && c <= b)
        .count();
    println!("b: {result:?}")
}

pub fn run() {
    let input = include_str!("../inputs/day-04.txt");
    run_a(input);
    run_b(input);
}
