use RPS::*;

enum RPS {
    // 1
    Rock,
    // 2
    Paper,
    // 3
    Scissors,
}

fn main() {
    let input = aoc_auto::input("2").unwrap();
    //    let mut split = input.lines().map(|s| s.split_whitespace());
    let mut split = input.split_whitespace();
    let mut result = 0;
    while let (Some(l), Some(r)) = (split.next(), split.next()) {
        let (left, right) = (char_to_rps(l), char_to_rps(r));
        // result += simulate(left, right);
        result += simulate_2(left, right);
    }
    println!("{}", result);
}

fn char_to_rps(input: &str) -> RPS {
    match input {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        _ => panic!(),
    }
}

// return right side's score for the round
fn simulate(left: RPS, right: RPS) -> u64 {
    match (left, right) {
        (Rock, Rock) => 1 + 3,
        (Rock, Paper) => 2 + 6,
        (Rock, Scissors) => 3 + 0,

        (Paper, Rock) => 1 + 0,
        (Paper, Paper) => 2 + 3,
        (Paper, Scissors) => 3 + 6,

        (Scissors, Rock) => 1 + 6,
        (Scissors, Paper) => 2 + 0,
        (Scissors, Scissors) => 3 + 3,
    }
}

fn simulate_2(left: RPS, right: RPS) -> u64 {
    match (left, right) {
        (Rock, Rock) => 3 + 0,
        (Rock, Paper) => 1 + 3,
        (Rock, Scissors) => 2 + 6,

        (Paper, Rock) => 1 + 0,
        (Paper, Paper) => 2 + 3,
        (Paper, Scissors) => 3 + 6,

        (Scissors, Rock) => 2 + 0,
        (Scissors, Paper) => 3 + 3,
        (Scissors, Scissors) => 1 + 6,
    }
}
