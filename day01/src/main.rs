use utils::utils;

fn main() {
    let path = format!("{}/resources/input.csv", env!("CARGO_MANIFEST_DIR"));
    let input = utils::read_and_parse(&path);

    let result = day01_1(&input);
    println!("Day 01 - 1: {:?}", result);
    let result2 = day01_2(&input);
    println!("Day 01 - 2: {:?}", result2);
}

fn day01_2(input: &Vec<i64>) -> Option<i64> {
    for a in input.iter() {
        for b in input.iter() {
            for c in input.iter() {
                if a + b + c == 2020 {
                    return Some(a * b * c)
                }
            }
        }
    }
    None
}

#[test]
fn day01_2_test() {
    let path = format!("{}/resources/test_input.csv", env!("CARGO_MANIFEST_DIR"));
    let test_input = utils::read_and_parse(&path);
    let test_result = day01_2(&test_input);
    assert_eq!(test_result, Some(241861950));
}

fn day01_1(input: &Vec<i64>) -> Option<i64> {
    for a in input.iter() {
        if let Some(b) = input
            .iter()
            .find(|n| *n + a == 2020) {
                return Some(a * b);
            }
    }
    None
}

#[test]
fn day01_1_test() {
    let path = format!("{}/resources/test_input.csv", env!("CARGO_MANIFEST_DIR"));
    let test_input = utils::read_and_parse(&path);
    let test_result = day01_1(&test_input);
    assert_eq!(test_result, Some(514579));
}
