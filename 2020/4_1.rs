use std::collections::HashSet;
use std::io::Read;

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
    let mut found = HashSet::new();
    for field in fields_from_passport(passport) {
        let field: Vec<&str> = field.split(':').collect();
        assert!(field.len() == 2);
        let field = field[0];
        found.insert(field);
    }

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in required.iter() {
        if !found.contains(field) {
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
