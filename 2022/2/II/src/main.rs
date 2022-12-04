use std::fs;

enum MoveSet {
    ROCK,
    PAPER,
    SCISSORS,
}

#[derive(Copy, Clone)]
enum MatchResult {
    LOSE,
    DRAW,
    WIN,
}

fn parse_separated_values_file(file_path: &str, separator: char) -> Vec<Vec<String>> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.split(separator).map(str::to_string).collect())
        .collect()
}

fn convert_letter_to_move(letter: char) -> MoveSet {
    match letter as i32 - 'A' as i32 {
        0 => MoveSet::ROCK,
        1 => MoveSet::PAPER,
        2 => MoveSet::SCISSORS,
        _ => panic!("Invalid move"),
    }
}

fn convert_letter_to_result(letter: char) -> MatchResult {
    match letter as i32 - 'X' as i32 {
        0 => MatchResult::LOSE,
        1 => MatchResult::DRAW,
        2 => MatchResult::WIN,
        _ => panic!("Invalid result"),
    }
}

fn get_result_score(result: MatchResult) -> i32 {
    match result {
        MatchResult::LOSE => 0,
        MatchResult::DRAW => 3,
        MatchResult::WIN => 6,
    }
}

fn get_strategic_move(opponent_move: MoveSet, result: MatchResult) -> MoveSet {
    match result {
        MatchResult::LOSE => match opponent_move {
            MoveSet::ROCK => MoveSet::SCISSORS,
            MoveSet::PAPER => MoveSet::ROCK,
            MoveSet::SCISSORS => MoveSet::PAPER,
        },
        MatchResult::WIN => match opponent_move {
            MoveSet::ROCK => MoveSet::PAPER,
            MoveSet::PAPER => MoveSet::SCISSORS,
            MoveSet::SCISSORS => MoveSet::ROCK,
        },
        MatchResult::DRAW => opponent_move,
    }
}

fn main() {
    let strategy_guide = parse_separated_values_file("../strategy_guide.txt", ' ');

    println!(
        "Total score: {}",
        strategy_guide
            .iter()
            .map(|round| {
                let opponent_move = convert_letter_to_move(round[0].chars().next().unwrap());
                let result = convert_letter_to_result(round[1].chars().next().unwrap());
                get_result_score(result) + get_strategic_move(opponent_move, result) as i32 + 1
            })
            .sum::<i32>()
    )
}
