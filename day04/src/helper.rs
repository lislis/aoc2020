use utils::utils;
use std::collections::HashMap;
use regex::Regex;

pub fn read_parse_passports(path: &str) -> Vec<HashMap<String, String>> {
    let content = utils::read_file(path);
    let lines = content.split("\n\n").collect::<Vec<&str>>();
    let values = lines.iter()
        .map(|x| create_passport(x))
        .collect::<Vec<HashMap<String, String>>>();
    values
}

pub fn create_passport(input: &str) -> HashMap<String, String> {
    let no_n_l = input.replace("\n", " ");
    let mut hm = HashMap::new();
    let _splits = no_n_l.split(" ")
        .map(|x| {
            let kv: Vec<&str> = x.split(":").collect();
            if kv.len() == 2 {
                hm.insert(kv[0].to_string(), kv[1].to_string());
            }
            x
        })
        .collect::<Vec<_>>();
    hm
}

pub fn validate_field_presence(hm: &HashMap<String, String>) -> bool {
    if (hm.contains_key("cid") && hm.len() == 8)
        || (!hm.contains_key("cid") && hm.len() == 7) {
            return true;
        } else {
            return false;
        }
}

pub fn validate_field_content(hm: &HashMap<String, String>) -> bool {
    let mut is_valid = true;

    match hm.get("byr").unwrap().parse::<i32>() {
        Ok(x) => {
            match is_in_range(x, 1920, 2002) {
                true => (),
                false => { is_valid = false; }
            }
        }
        _ => { is_valid = false; }
    }
    match hm.get("iyr").unwrap().parse::<i32>() {
        Ok(x) => {
            match is_in_range(x, 2010, 2020) {
                true => (),
                false => { is_valid = false; }
            }
        }
        _ => { is_valid = false; }
    }
    match hm.get("eyr").unwrap().parse::<i32>() {
        Ok(x) => {
            match is_in_range(x, 2020, 2030) {
                true => (),
                false => { is_valid = false; }
            }
        }
        _ => { is_valid = false; }
    }


    // imcomplete
    let hgt = hm.get("hgt").unwrap();

    {
        let re = Regex::new(r"[\d]{2,3}[cm|in]$").unwrap();
        if re.is_match(hgt) {
            let cm: Vec<&str> = hgt.split("cm").collect();
            if cm.len() > 1 {
                let h = cm[0].parse::<i32>().unwrap();
                match is_in_range(h, 150, 193) {
                    true => (),
                    false => { is_valid = false; }
                }
            }
            let inch: Vec<&str> = hgt.split("in").collect();
            if inch.len() > 1 {
                let h = inch[0].parse::<i32>().unwrap();
                match is_in_range(h, 59, 76) {
                    true => (),
                    false => { is_valid = false; }
                }
            }
        } else {
            is_valid = false;
        }
    }

    let hcl = hm.get("hcl").unwrap();
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    if !re.is_match(hcl) { is_valid = false; }

    let ecl = hm.get("ecl").unwrap();
    if ecl == "amb" ||
        ecl == "blu" ||
        ecl == "brn" ||
        ecl == "gry" ||
        ecl == "grn" ||
        ecl == "hzl" ||
        ecl == "oth" {
            ()
        } else { is_valid = false; }

    let pid = hm.get("pid").unwrap();
    let re = Regex::new(r"^0[0-9]{8}$").unwrap();
    if !re.is_match(pid) { is_valid = false; }


    is_valid
}


fn is_in_range(value: i32, min: i32, max: i32) -> bool {
    if value >= min && value < max {
        return true;
    } else { return false; }
}
