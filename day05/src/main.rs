mod helper;

fn main() {
    let path = format!("{}/resources/input", env!("CARGO_MANIFEST_DIR"));
    let boarding_passes = helper::read_parse_boarding_passes(&path);

    let largest_id = day05_1(&boarding_passes);
    println!("Day 05 - 1: {:?}", largest_id);

    let own_pass = day05_2(&boarding_passes);
    println!("Day 05 - 2: {:?}", own_pass);
}

fn day05_1(input: &Vec<String>) -> i32 {
    let largest_id = input.iter()
        .map(|x| helper::get_boarding_tuple(x))
        .map(|x| x.2)
        .max_by(|x, y| x.cmp(y)).unwrap();

    largest_id
}

fn day05_2(input: &Vec<String>) -> i32 {
    let passes = input.iter()
        .map(|x| helper::get_boarding_tuple(x))
        .map(|x| x.2)
        .collect::<Vec<i32>>();

    let largest = passes.iter().max_by(|x, y| x.cmp(y)).unwrap();
    let mut needle: i32 = 0;

    for r in (0..127).into_iter() {
        for c in (0..7).into_iter() {
            let id: i32 = r * 8 + c;

            if id < *largest {
                if r != 0 && r != 127 {
                    let foo  = passes.iter().find(|&i| i == &id);
                    match foo {
                        Some(_) => (),
                        None => { needle = id; }
                    }
                }
            }
        }
    }
    needle
}
