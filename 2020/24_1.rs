use std::collections::HashMap;
use std::io::Read;

fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut idx = 0;
    let s: Vec<char> = s.chars().collect();
    while idx < s.len() {
        match s[idx] {
            's' | 'n' => {
                out.push(s[idx..idx + 2].iter().collect());
                idx += 1;
            }
            _ => out.push(s[idx].to_string()),
        };

        idx += 1;
    }

    out
}

fn parse_tile(s: &str) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    let map: HashMap<&str, (i32, i32)> = [
        ("se", (1, 1)),
        ("sw", (-1, 1)),
        ("ne", (1, -1)),
        ("nw", (-1, -1)),
        ("e", (2, 0)),
        ("w", (-2, 0)),
    ]
    .iter()
    .cloned()
    .collect();

    for s in tokenize(s) {
        let movement = map[&s[..]];
        x += movement.0;
        y += movement.1;
    }

    (x, y)
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for s in buffer.trim().split('\n') {
        let tile = parse_tile(s);
        let x = map.entry(tile).or_insert(0);
        *x = 1 - *x;
    }

    println!("{}", map.values().sum::<i32>());
}
