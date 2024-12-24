
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Should have been able to read the file");

    let mut left_list: Vec<&str> = Vec::new();
    let mut right_list: Vec<&str> = Vec::new();
    let clean_list = contents.split_whitespace();
    let full_list: Vec<&str> = clean_list.collect();
    for string in full_list.iter().step_by(2) {
        left_list.push(string);
    }

    for string in full_list.iter().skip(1).step_by(2) {
        right_list.push(string);
    }

    assert_eq!(left_list.len(), right_list.len());
    println!("Left Vector: {:?}", left_list);
    println!("Right Vector: {:?}", right_list);
}
