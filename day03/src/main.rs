mod helper;

fn main() {
    let path = format!("{}/resources/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = helper::read_parse_map(&path);

    let result = day03_1(&input);
    println!("Day 03 - 1: {:?}", result);

    let result2 = day03_2(&input);
    println!("Day 03 - 2: {:?}", result2);
}


fn day03_1(input: &Vec<Vec<char>>) -> usize {
    helper::slope_runner(input, helper::Vector {x: 3, y: 1})
}

#[test]
fn day03_1_test() {
    let path = format!("{}/resources/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let test_input = helper::read_parse_map(&path);
    let test_result = day03_1(&test_input);
    assert_eq!(test_result, 7);
}

fn day03_2(input: &Vec<Vec<char>>) -> usize {
    let slopes = vec!(helper::Vector {x: 1, y: 1},
                      helper::Vector {x: 3, y: 1},
                      helper::Vector {x: 5, y: 1},
                      helper::Vector {x: 7, y: 1},
                      helper::Vector {x: 1, y: 2});
    return slopes.iter()
        .map(|x| helper::slope_runner(input, *x))
        .fold(1, |sum, x| sum * x)
}

#[test]
fn day03_2_test() {
    let path = format!("{}/resources/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let test_input = helper::read_parse_map(&path);
    let test_result = day03_2(&test_input);
    assert_eq!(test_result, 336);
}
