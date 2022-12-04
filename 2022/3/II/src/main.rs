use std::fs;

fn get_rucksacks() -> Vec<Vec<String>> {
    fs::read_to_string("../rucksacks.txt")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|group| group.to_vec())
        .collect()
}

fn main() {
    println!(
        "Sum of priorities: {}",
        get_rucksacks()
            .iter()
            .map(|group| {
                let badge = group[0]
                    .chars()
                    .filter(|item| group[1].contains(*item) && group[2].contains(*item))
                    .next()
                    .unwrap();

                badge as i32 - 'A' as i32
                    + if badge.is_uppercase() {
                        2 + 'Z' as i32 - 'A' as i32
                    } else {
                        -6 + 'a' as i32 - 'z' as i32
                    }
            })
            .sum::<i32>()
    )
}
