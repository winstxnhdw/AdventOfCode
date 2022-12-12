use std::collections::HashMap;
use std::fs;

fn get_next_start_index(substring: &str) -> i32 {
    let mut character_map = HashMap::new();

    for (i, c) in substring.chars().enumerate() {
        if character_map.contains_key(&c) {
            return 1 + *character_map.get(&c).unwrap() as i32;
        }

        character_map.insert(c, i);
    }

    return -1;
}

fn get_start_of_packet_marker() -> i32 {
    let datastream_buffer = fs::read_to_string("../datastream_buffer.txt").unwrap();
    let slice_length = 4;
    let mut start_slice: i32 = 0;

    loop {
        if start_slice + slice_length > datastream_buffer.len() as i32 {
            panic!("No start-of-packet marker found!");
        }

        let end_slice: i32 = start_slice + slice_length;
        let start_of_packet_marker = &datastream_buffer[start_slice as usize..end_slice as usize];
        let next_start_index = get_next_start_index(start_of_packet_marker);

        if next_start_index == -1 {
            return end_slice;
        }

        start_slice += next_start_index;
    }
}

fn main() {
    println!(
        "Characters before the first start-of-packet marker: {}",
        get_start_of_packet_marker()
    );
}
