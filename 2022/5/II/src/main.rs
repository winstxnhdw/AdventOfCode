use std::fs;

struct Schematic {
    drawing: String,
    procedure: String,
}

fn get_schematic() -> Schematic {
    let schematic_content = fs::read_to_string("../schematic.txt")
        .unwrap()
        .split("\n\n")
        .map(str::to_string)
        .collect::<Vec<String>>();

    Schematic {
        drawing: schematic_content[0].clone(),
        procedure: schematic_content[1].clone(),
    }
}

fn transpose<T>(vector: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = vector[0].len();
    let mut iters: Vec<_> = vector.into_iter().map(|n| n.into_iter()).collect();

    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn get_drawing_vector(drawing: &str) -> Vec<Vec<char>> {
    let drawing_lines = drawing
        .lines()
        .map(|line| line.chars().collect())
        .rev()
        .collect::<Vec<Vec<char>>>();

    transpose(drawing_lines)
        .into_iter()
        .filter_map(|c| match c.first().unwrap().is_digit(10) {
            true => Some(
                c.into_iter()
                    .skip(1)
                    .filter(|c| c.is_alphabetic())
                    .collect(),
            ),
            _ => None,
        })
        .collect()
}

fn get_procedure_vector(procedure: &str) -> Vec<Vec<usize>> {
    procedure
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|word| match word.parse::<usize>() {
                    Ok(stack_number) => Some(stack_number),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn rearrange(
    drawing_vector: &Vec<Vec<char>>,
    procedure_vector: &Vec<Vec<usize>>,
) -> Vec<Vec<char>> {
    let mut rearranged_stack: Vec<Vec<char>> = drawing_vector.to_vec();

    for procedure in procedure_vector {
        let chosen_stack = procedure[1] - 1;
        let popped_length = rearranged_stack[chosen_stack].len() - procedure[0];
        let popped_crates = rearranged_stack[chosen_stack]
            .drain(popped_length..)
            .collect::<Vec<char>>();
        rearranged_stack[procedure[2] - 1].extend(popped_crates);
    }

    rearranged_stack
}

fn main() {
    let scheme = get_schematic();
    let drawing_vector = get_drawing_vector(&scheme.drawing);
    let procedure_vector = get_procedure_vector(&scheme.procedure);
    let rearranged_stack = rearrange(&drawing_vector, &procedure_vector);

    println!(
        "Crates that end up on top of each stack: {:?}",
        rearranged_stack
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    );
}
