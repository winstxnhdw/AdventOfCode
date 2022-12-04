use std::{collections::HashSet, fs};

fn get_rucksacks() -> Vec<Vec<Vec<char>>> {
    fs::read_to_string("../rucksacks.txt")
        .unwrap()
        .lines()
        .map(|rucksack| {
            rucksack
                .chars()
                .collect::<Vec<char>>()
                .chunks(rucksack.len() / 2)
                .map(|chunk| chunk.to_vec())
                .collect()
        })
        .collect()
}

fn main() {
    println!(
        "Sum of priorities: {}",
        get_rucksacks()
            .iter()
            .map(|rucksack| {
                let compartment_one = rucksack[0].iter().collect::<HashSet<&char>>();
                let compartment_two = rucksack[1].iter().collect::<HashSet<&char>>();
                let erroneous_item = compartment_one
                    .intersection(&compartment_two)
                    .next()
                    .unwrap() as &char;

                *erroneous_item as i32 - 'A' as i32
                    + if erroneous_item.is_uppercase() {
                        2 + 'Z' as i32 - 'A' as i32
                    } else {
                        -6 + 'a' as i32 - 'z' as i32
                    }
            })
            .sum::<i32>()
    )
}
