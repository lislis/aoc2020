use utils::utils;

fn read_parse_travel_groups(path: &str) -> Vec<String> {
    let content = utils::read_file(path);
    let groups = content.split("\n\n")
        .map(|x| x.split('\n').collect::Vec<&str>())
        .collect::<Vec<Vec<&str>>>();
    groups
}
