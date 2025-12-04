use std::fs;

pub fn read_input(day: u8) -> String {
    return fs::read_to_string(format!("./input/day_{}.txt", day)).expect("Failed to load input");
}