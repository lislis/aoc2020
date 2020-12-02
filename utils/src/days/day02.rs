use regex::Regex;
use crate::utils;


#[derive(Debug)]
pub struct PasswordRecord {
    pub min_occur: i32,
    pub max_occur: i32,
    pub letter: char,
    pub password: String
}

impl PasswordRecord {
    pub fn is_valid(&self) -> bool {
        let v: Vec<&str> = self.password.matches(self.letter).collect();
        if v.len() >= self.min_occur as usize
            && v.len() <= self.max_occur as usize {
                return true
            }
        false
    }

    pub fn is_valid_other_rule(&self) -> bool {
        let mut first = false;
        let mut last = false;
        if let Some(fst) = self.password.chars().nth(self.min_occur as usize -1) {
            if self.letter == fst {
                first = true;
            }
        };
        if let Some(lst) = self.password.chars().nth(self.max_occur as usize -1) {
            if self.letter == lst {
                last = true;
            }
        };

        first ^ last
    }
}

fn split_password_record(line: &str) -> PasswordRecord {
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]{1}): ([a-z]+)").expect("Invalid regex");
    let cap = re.captures(line).expect("No match or something");

    PasswordRecord {
        min_occur: cap[1].parse::<i32>().expect("Could not parse int"),
        max_occur: cap[2].parse::<i32>().expect("Could not parse int"),
        letter: cap[3].parse::<char>().unwrap(),
        password: cap[4].to_string()
    }
}

pub fn read_parse_passwords(path: &str) -> Vec<PasswordRecord> {
    let content = utils::read_file(path);
    let lines = content.split("\n").collect::<Vec<&str>>();

    let values = lines.iter()
        .take(lines.len() -1)
        .map(|x| split_password_record(x))
        .collect::<Vec<PasswordRecord>>();
    values
}
