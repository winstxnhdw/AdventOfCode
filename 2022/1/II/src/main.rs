use itertools::Itertools;
use std::fs;

fn get_calorie_per_inventory() -> Vec<i32> {
    let calorie_inventories: String = fs::read_to_string("../calorie_inventory_list.txt").unwrap();
    let calorie_inventories_parsed = calorie_inventories.split("\n\n").map(|inventory| {
        inventory
            .lines()
            .map(|calorie| calorie.parse::<i32>().unwrap())
    });

    calorie_inventories_parsed
        .map(|inventory| inventory.sum::<i32>())
        .collect()
}

fn main() {
    println!(
        "Highest calorie inventory: {}",
        get_calorie_per_inventory()
            .iter()
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
    );
}
