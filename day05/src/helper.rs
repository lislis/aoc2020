use utils::utils;

pub fn read_parse_boarding_passes(path: &str) -> Vec<String>{
    let content = utils::read_file(path);
    let lines = content.split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    lines
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoardingTuple(pub i32, pub i32, pub i32);

pub fn get_boarding_tuple(input: &str) -> BoardingTuple {
    let mut r_min = 0;
    let mut r_max = 127;
    let mut r = 0;

    let mut c_min = 0;
    let mut c_max = 7;
    let mut c = 0;

    for (k, a) in input.chars().into_iter().enumerate() {
        match a {
            'F' => {
                r_max = r_min + ((r_max - r_min) / 2);
                if k == 6 { r = r_min; }
            },
            'B' => {
                r_min = r_max - ((r_max - r_min) / 2);
                if k == 6 { r = r_max; }
            },
            'L' => {
                c_max = c_min + ((c_max - c_min) / 2);
                if k == 9 { c = c_min; }
            },
            'R' => {
                c_min = c_max - ((c_max - c_min) / 2);
                if k == 9 { c = c_max; }
            },
            _ => panic!("Danger Will Robinson!")
        }
    }

    let id = (r * 8) + c;

    BoardingTuple(r, c, id)
}

#[test]
fn get_boarding_tuple_test() {
    assert_eq!(get_boarding_tuple("FBFBBFFRLR"), BoardingTuple(44, 5, 357));
    assert_eq!(get_boarding_tuple("BFFFBBFRRR"), BoardingTuple(70, 7, 567));
    assert_eq!(get_boarding_tuple("FFFBBBFRRR"), BoardingTuple(14, 7, 119));
    assert_eq!(get_boarding_tuple("BBFFBBFRLL"), BoardingTuple(102, 4, 820));
}
