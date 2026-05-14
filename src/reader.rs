use std::fs;

pub fn read(input: &str) -> String {
    if input.ends_with(".txt") {
        fs::read_to_string(input)
            .expect("Could not read file")
    } else {
        input.to_string()
    }
}