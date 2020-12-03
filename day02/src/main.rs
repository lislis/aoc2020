mod helper;

fn main() {
    let path = format!("{}/resources/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = helper::read_parse_passwords(&path);
    let result = day02_1(&input);
    println!("Day 02 - 1: {:?}", result);
    let result2 = day02_2(&input);
    println!("Day 02 - 2: {:?}", result2);
}


fn day02_1(input: &Vec<helper::PasswordRecord>) -> usize {
    let v: Vec<&helper::PasswordRecord> = input.iter()
        .filter(|x| x.is_valid())
        .collect();
    v.len()
}

#[test]
fn day02_1_test() {
    let path = format!("{}/resources/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let test_input = helper::read_parse_passwords(&path);
    let test_result = day02_1(&test_input);
    assert_eq!(test_result, 2);
}


fn day02_2(input: &Vec<helper::PasswordRecord>) -> usize {
    let v: Vec<&helper::PasswordRecord> = input.iter()
        .filter(|x| x.is_valid_other_rule())
        .collect();
    v.len()
}

#[test]
fn day02_2_test() {
    let path = format!("{}/resources/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let test_input = helper::read_parse_passwords(&path);
    let test_result = day02_2(&test_input);
    assert_eq!(test_result, 1);
}
