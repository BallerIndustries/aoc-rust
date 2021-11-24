use std::fs;

pub fn read_all_text(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return contents
}