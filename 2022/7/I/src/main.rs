use std::collections::HashMap;
use std::fs;

fn get_terminal_output() -> Vec<String> {
    fs::read_to_string("../terminal_output.txt")
        .unwrap()
        .lines()
        .skip(2)
        .map(str::to_string)
        .collect()
}

fn handle_cd(path: &mut Vec<String>, argument: String) {
    if argument == ".." {
        path.pop();
    } else {
        path.push(argument);
    }
}

fn handle_size(
    directory_sizes: &mut HashMap<String, usize>,
    working_directory: &Vec<String>,
    size: usize,
) {
    for i in 1..working_directory.len() + 1 {
        let path = working_directory[0..i].join("/");

        directory_sizes.insert(
            path.clone(),
            size + if directory_sizes.contains_key(&path) {
                directory_sizes.get(&path).unwrap()
            } else {
                &0
            },
        );
    }
}

fn get_directory_sizes() -> HashMap<String, usize> {
    let terminal_output = get_terminal_output();
    let mut working_directory: Vec<String> = vec!["/".to_string()];
    let mut directory_sizes: HashMap<String, usize> = HashMap::new();

    for output in terminal_output {
        if output.starts_with("dir") || output.starts_with("$ ls") {
            continue;
        }

        let output_splitted = output.split_whitespace();

        if output.starts_with("$ cd") {
            handle_cd(
                &mut working_directory,
                output_splitted.last().unwrap().to_string(),
            );
        } else {
            handle_size(
                &mut directory_sizes,
                &working_directory,
                output_splitted.clone().next().unwrap().parse().unwrap(),
            );
        }
    }

    directory_sizes
}

fn main() {
    println!(
        "Total size of directories with a size of at most 100000: {}",
        get_directory_sizes()
            .values()
            .filter(|size| **size <= 1e5 as usize)
            .sum::<usize>()
    );
}
