use std::fs;

fn get_calorie_per_inventory() -> Vec<i32> {
    let calorie_inventories: String = fs::read_to_string("../calorie_inventory_list.txt").unwrap();
    let calorie_inventories_parsed = calorie_inventories.split("\n\n").map(|inventory| {
        inventory
            .split('\n')
            .map(|calorie| calorie.parse::<i32>().unwrap())
    });

    calorie_inventories_parsed
        .map(|inventory| inventory.sum::<i32>())
        .collect()
}

fn main() {
    let mut three_largest_calorie_inventories = get_calorie_per_inventory();
    three_largest_calorie_inventories.sort();
    three_largest_calorie_inventories.reverse();
    three_largest_calorie_inventories.truncate(3);

    println!(
        "Highest calorie inventory: {}",
        three_largest_calorie_inventories.iter().sum::<i32>()
    );
}
