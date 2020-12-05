mod helper;
use std::collections::HashMap;


fn main() {
    let path = format!("{}/resources/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = helper::read_parse_passports(&path);
    let result = day04_1(&input);
    println!("Day 04 - 1: {:?}", result);
}


fn day04_1(input: &Vec<HashMap<String, String>>) -> usize {
    let valid_passports: Vec<&HashMap<String, String>> = input.iter()
        .filter(|hm| helper::validate_field_presence(hm))
        .collect();
    valid_passports.len()
}


#[test]
fn day04_1_test() {
    let path = format!("{}/resources/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let test_input = helper::read_parse_passports(&path);
    let test_result = day04_1(&test_input);
    assert_eq!(test_result, 2);
}

fn day04_2(input: &Vec<HashMap<String, String>>) -> usize {
    let valid_passports: Vec<&HashMap<String, String>> = input.iter()
        .filter(|hm| helper::validate_field_presence(hm))
        .filter(|hm| helper::validate_field_content(hm))
        .collect();
    valid_passports.len()
}

#[test]
fn day04_2_test() {
    let valid_path = format!("{}/resources/test_input_valid.txt", env!("CARGO_MANIFEST_DIR"));
    let valid_input = helper::read_parse_passports(&valid_path);
    let valid_test_result = day04_2(&valid_input);
    assert_eq!(valid_test_result, 4);

    let invalid_path = format!("{}/resources/test_input_invalid.txt", env!("CARGO_MANIFEST_DIR"));
    let invalid_input = helper::read_parse_passports(&invalid_path);
    let invalid_test_result = day04_2(&invalid_input);
    assert_eq!(invalid_test_result, 0);
}
