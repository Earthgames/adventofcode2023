use std::fs;
pub mod one;

fn get_input(day: u8) -> String {
    fs::read_to_string(format!("input/input{}", day)).expect("could not read file")
}
