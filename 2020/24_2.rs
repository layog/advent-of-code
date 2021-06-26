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

fn simulate(tiles: &mut HashMap<(i32, i32), i8>) {
    let mut n_black_count: HashMap<(i32, i32), i8> = HashMap::new();
    let adjacent_tiles: [(i32, i32); 6] = [(2, 0), (-2, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (tile, &is_black) in tiles.iter() {
        if !n_black_count.contains_key(tile) {
            n_black_count.insert(*tile, 0);
        }

        if is_black == 1 {
            for &(x, y) in adjacent_tiles.iter() {
                let otherx = tile.0 + x;
                let othery = tile.1 + y;

                let inc = n_black_count.entry((otherx, othery)).or_insert(0);
                *inc += 1;
            }
        }
    }

    for (tile, black_count) in n_black_count.into_iter() {
        let is_black = tiles.entry(tile).or_insert(0);

        if *is_black == 1 {
            if black_count == 0 || black_count > 2 {
                *is_black = 0;
            }
        } else if black_count == 2 {
            *is_black = 1;
        }
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut map: HashMap<(i32, i32), i8> = HashMap::new();

    for s in buffer.trim().split('\n') {
        let tile = parse_tile(s);
        let x = map.entry(tile).or_insert(0);
        *x = 1 - *x;
    }

    let mut days = 100;
    while days != 0 {
        simulate(&mut map);
        days -= 1;
    }

    println!("{}", map.values().map(|&x| x as i32).sum::<i32>());
}
