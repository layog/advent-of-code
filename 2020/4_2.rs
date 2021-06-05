use std::collections::HashMap;
use std::io::Read;

macro_rules! unwrap {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return false,
        }
    };
}

fn validate_byr(input: &str) -> bool {
    let i: i32 = unwrap!(input.parse());
    (1920..=2002).contains(&i)
}

fn validate_iyr(input: &str) -> bool {
    let i: i32 = unwrap!(input.parse());
   (2010..=2020).contains(&i)
}

fn validate_eyr(input: &str) -> bool {
    let i: i32 = unwrap!(input.parse());
   (2020..=2030).contains(&i)
}

fn validate_hgt(input: &str) -> bool {
    match input.trim() {
        x if x.ends_with("cm") => {
            let i: i32 = unwrap!(x[..x.len() - 2].parse());
            (150..=193).contains(&i)
        }
        x if x.ends_with("in") => {
            let i: i32 = unwrap!(x[..x.len() - 2].parse());
            (59..=76).contains(&i)
        }
        _ => false,
    }
}

fn validate_hcl(input: &str) -> bool {
    let input = input.trim();
    if !input.starts_with('#') {
        return false;
    }

    for c in input[1..].chars() {
        if !('a'..='f').contains(&c) && !('0'..='9').contains(&c) {
            return false;
        }
    }
    true
}

fn validate_ecl(input: &str) -> bool {
    let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid.contains(&input.trim())
}

fn validate_pid(input: &str) -> bool {
    if input.trim().len() != 9 {
        return false;
    }
    unwrap!(input.parse::<i32>());
    true
}

fn fields_from_passport(passport: &str) -> Vec<&str> {
    let fields: Vec<&str> = passport
        .split('\n')
        .map(|x| x.split(' '))
        .flatten()
        .collect();
    fields
        .into_iter()
        .filter(|x| !x.is_empty() && (x.contains(':')))
        .collect()
}

fn validate_passport(passport: &str) -> bool {
    let mut found: HashMap<&str, &str> = HashMap::new();
    for field in fields_from_passport(passport) {
        let field: Vec<&str> = field.split(':').collect();
        assert!(field.len() == 2);
        found.insert(field[0], field[1]);
    }

    type FnTuple<'a> = (&'a str, fn(&str) -> bool);
    let required: [FnTuple; 7] = [
        ("byr", validate_byr),
        ("iyr", validate_iyr),
        ("eyr", validate_eyr),
        ("hgt", validate_hgt),
        ("hcl", validate_hcl),
        ("ecl", validate_ecl),
        ("pid", validate_pid),
    ];

    for (field, validator) in required.iter() {
        if !found.contains_key(field) {
            return false;
        }
        if !validator(found[field]) {
            return false;
        }
    }

    true
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut valid_count = 0;
    for passport in buffer.split("\n\n") {
        if passport.trim().is_empty() {
            continue;
        }

        valid_count += validate_passport(passport) as usize;
    }

    println!("{}", valid_count);
    Ok(())
}
