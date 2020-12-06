use utils::utils;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let path = format!("{}/resources/input.txt", env!("CARGO_MANIFEST_DIR"));
    let result = day06_1(&path);
    println!("Day 06 - 1: {:?}", result);

    let result2 = day06_2(&path);
    println!("Day 06 - 2: {:?}", result2);
}


fn day06_1(input: &str) -> usize {
    let content = utils::read_file(input);
    let group_length = content.split("\n\n")
        .map(|x| {
            let mut hs: HashSet<char> = HashSet::new();
            x.split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|y| y.chars().map(|z| {
                    hs.insert(z);
                    z
                }).collect::<Vec<char>>())
                .for_each(drop);
            hs
        })
        .map(|x| x.len())
        .fold(0, |acc, x| acc + x);
    group_length
}

#[test]
fn day06_1_test() {
    let path = format!("{}/resources/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let test_result = day06_1(&path);
    assert_eq!(test_result, 11);
}

fn day06_2(input: &str) -> usize {
    let content = utils::read_file(input);
    content.split("\n\n")
        .map(|x| {
            let group = x.split('\n')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();

            let mut hm: HashMap<char, i32> = HashMap::new();
            group.iter().for_each(|y| {
                y.chars().for_each(|z| {
                    let counter = hm.entry(z).or_insert(0);
                    *counter += 1;
                })
            });

            hm.iter().fold(0, |acc, (_k, v)| {
                if v / group.len() as i32 == 1 {
                    acc + 1
                } else { acc }
            })
        })
        .fold(0, |acc, x| acc + x)
}

#[test]
fn day06_2_test() {
    let path = format!("{}/resources/test_input.txt", env!("CARGO_MANIFEST_DIR"));
    let test_result = day06_2(&path);
    assert_eq!(test_result, 6);
}
