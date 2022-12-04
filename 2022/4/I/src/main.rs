use std::fs;

fn get_section_assignments() -> Vec<Vec<Vec<i32>>> {
    fs::read_to_string("../section_assignments.txt")
        .unwrap()
        .lines()
        .map(|pair| {
            pair.split(',')
                .map(|assignments| {
                    assignments
                        .split('-')
                        .map(|section| section.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn check_if_assignments_fully_contain(pair_assignment: &Vec<Vec<i32>>) -> bool {
    let mut assignment_one_sortable = pair_assignment[0].clone();
    let mut assignment_two_sortable = pair_assignment[1].clone();
    assignment_one_sortable.sort();
    assignment_two_sortable.sort();

    if assignment_one_sortable[0] < assignment_two_sortable[0] {
        assignment_one_sortable[1] >= assignment_two_sortable[1]
    } else if assignment_one_sortable[0] > assignment_two_sortable[0] {
        assignment_one_sortable[1] <= assignment_two_sortable[1]
    } else {
        true
    }
}

fn main() {
    println!(
        "Number of pairs that have assignments that fully contain the other: {}",
        get_section_assignments()
            .iter()
            .map(check_if_assignments_fully_contain)
            .filter(|b| *b)
            .count()
    );
}
