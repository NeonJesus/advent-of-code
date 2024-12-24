
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Should have been able to read the file");

    let mut left_column: Vec<&str> = Vec::new();
    let mut right_column: Vec<&str> = Vec::new();
    let clean_list = contents.split_whitespace();
    let full_list: Vec<&str> = clean_list.collect();
    for string in full_list.iter().step_by(2) {
        left_column.push(string);
    }

    for string in full_list.iter().skip(1).step_by(2) {
        right_column.push(string);
    }

    assert_eq!(left_column.len(), right_column.len());
    // println!("Left Vector: {:?}", left_column);
    // println!("Right Vector: {:?}", right_column);
    left_column.sort();
    right_column.sort();

    for (index, val) in left_column.iter().enumerate() {
        println!("Index : {index} Value: {val}");
    }
}
