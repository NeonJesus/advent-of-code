use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Should have been able to read the file");

    // let mut left_list: Vec<&str> = Vec::new();
    let clean_list = contents.split_whitespace();
    let left_list: Vec<&str> = clean_list.collect();
    for string in &left_list {
        println!("With text:\n{string}");
    }
}
