use std::fs;

#[derive(PartialEq, Copy, Clone)]
enum MoveSet {
    ROCK,
    PAPER,
    SCISSORS,
}

fn parse_separated_values_file(file_path: &str, separator: char) -> Vec<Vec<String>> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.split(separator).map(str::to_string).collect())
        .collect()
}

fn convert_letter_to_move(letter: char) -> MoveSet {
    let letter_ascii = letter as i32;

    let move_number = letter_ascii
        - if letter_ascii > 'C' as i32 {
            'X' as i32
        } else {
            'A' as i32
        };

    match move_number {
        0 => MoveSet::ROCK,
        1 => MoveSet::PAPER,
        2 => MoveSet::SCISSORS,
        _ => panic!("Invalid move"),
    }
}

fn get_rock_paper_scissors_result(player_move: MoveSet, opponent_move: MoveSet) -> i32 {
    if player_move == opponent_move {
        return 3;
    }

    let lose = 0;
    let win = 6;

    match (player_move, opponent_move) {
        (MoveSet::ROCK, MoveSet::PAPER) => lose,
        (MoveSet::ROCK, MoveSet::SCISSORS) => win,
        (MoveSet::PAPER, MoveSet::ROCK) => win,
        (MoveSet::PAPER, MoveSet::SCISSORS) => lose,
        (MoveSet::SCISSORS, MoveSet::ROCK) => lose,
        (MoveSet::SCISSORS, MoveSet::PAPER) => win,
        _ => panic!("Invalid move"),
    }
}

fn main() {
    let strategy_guide = parse_separated_values_file("../strategy_guide.txt", ' ');

    println!(
        "Total score: {}",
        strategy_guide
            .iter()
            .map(|round| {
                let player_move = convert_letter_to_move(round[1].chars().next().unwrap());
                let opponent_move = convert_letter_to_move(round[0].chars().next().unwrap());
                player_move as i32 + 1 + get_rock_paper_scissors_result(player_move, opponent_move)
            })
            .sum::<i32>()
    )
}
