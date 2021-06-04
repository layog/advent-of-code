use std::io::Read;

fn split(string: &str, at: char) -> (&str, &str) {
    let x: Vec<&str> = string.split(at).collect();
    assert!(x.len() == 2);
    (x[0], x[1])
}

fn validate(line: &str) -> bool {
    let (bounds, word) = split(line, ':');
    let (bounds, c) = split(bounds.trim(), ' ');
    let (min, max) = split(bounds.trim(), '-');

    let first: usize = min.parse().unwrap();
    let second: usize = max.parse().unwrap();
    let c: char = c.trim().parse().unwrap();
    let word = word.trim();

    if (word.len() < first) || (word.len() < second) {
        return false;
    }

    (word.chars().nth(first - 1).unwrap() == c)^(word.chars().nth(second - 1).unwrap() == c)
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let lines: Vec<&str> = buffer.trim_end().split('\n').collect();

    let mut ans: i32 = 0;
    for line in lines {
        ans += validate(line) as i32;
    }
    println!("{}", ans);

    Ok(())
}
