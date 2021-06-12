use std::collections::HashMap;
use std::io::Read;

// Some experiments -
// 1. Edges and their reverse have mostly two tiles in common
// 2. The boundary edges will not share edge and interestingly their reverse does not share as
//    well. So, it helps in identifying the boundaries.
// 3. The corners will have 2 edges with no common edge.

fn edges(tile: &[&str]) -> Vec<String> {
    let mut v = Vec::new();
    v.push(tile[0].to_string());
    v.push(tile[tile.len() - 1].to_string());
    v.push(tile.iter().map(|&x| x.chars().next().unwrap()).collect());
    v.push(tile.iter().map(|&x| x.chars().last().unwrap()).collect());

    let mut reversed = Vec::new();
    for x in v.iter() {
        reversed.push(x.chars().rev().collect::<String>());
    }

    v.append(&mut reversed);

    v
}

fn parse_tile(s: &str) -> (u64, Vec<&str>) {
    let v: Vec<&str> = s.trim().split('\n').map(|x| x.trim()).collect();
    let x: u64 = v[0][..v[0].len() - 1].split(' ').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    (x, (&v[1..]).to_vec())
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut tiles = Vec::new();
    let mut ids = Vec::new();
    for s in buffer.trim().split("\n\n") {
        let (i, tile) = parse_tile(s);
        ids.push(i);
        tiles.push(tile);
    }

    let mut h: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, tile) in tiles.iter().enumerate() {
        for edge in edges(tile) {
            let x = h.entry(edge).or_insert_with(Vec::new);
            x.push(i);
        }
    }

    let mut corners = Vec::new();
    for (i, tile) in tiles.iter().enumerate() {
        let mut count: u8 = 0;
        for edge in edges(tile) {
            count += (h[&edge].len() == 1) as u8;
        }

        if count == 4 {
            corners.push(i);
        }
    }

    let mut ans = 1;
    for i in corners {
        ans *= ids[i];
    }
    println!("{}", ans);
}
