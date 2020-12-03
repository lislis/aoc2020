use utils::utils;

pub fn read_parse_map(path: &str) -> Vec<Vec<char>> {
    let content = utils::read_file(path);
    let lines = content.split("\n").collect::<Vec<&str>>();
    let values = lines.iter()
        .take(lines.len() -1)
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    values
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: usize,
    pub y: usize
}

fn add_vec(one: Vector, other: Vector, ver_bound: usize) -> Vector {
    let x = (one.x + other.x) % ver_bound;
    let y = one.y + other.y;
    Vector {x: x, y: y}
}

pub fn slope_runner(input: &Vec<Vec<char>>, slope: Vector) -> usize {
    let hor_max = input.len();
    let ver_max = input[0].len();

    let mut tree_counter: usize = 0;
    let mut current_point = Vector {x: 0, y: 0};

    while current_point.y < hor_max {
        if input[current_point.y][current_point.x] == '#' {
            tree_counter += 1;
        }
        current_point = add_vec(current_point, slope, ver_max);
    }
    tree_counter
}
