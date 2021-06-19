use std::collections::{HashMap, HashSet};
use std::io::Read;

// Some experiments -
// 1. Edges and their reverse have mostly two tiles in common
// 2. The boundary edges will not share edge and interestingly their reverse does not share as
//    well. So, it helps in identifying the boundaries.
// 3. The corners will have 2 edges with no common edge.
// 4. No edge is common anywhere which will help in identifying all the tiles

#[derive(Debug)]
enum Edge {
    Left,
    Up,
}

fn parse_tile(s: &str) -> (u64, Vec<&str>) {
    let v: Vec<&str> = s.trim().split('\n').map(|x| x.trim()).collect();
    let x: u64 = v[0][..v[0].len() - 1].split(' ').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    (x, (&v[1..]).to_vec())
}

fn edges<T: AsRef<str> + std::fmt::Display>(tile: &[T]) -> Vec<String> {
    let tile: Vec<&str> = tile.iter().map(|x| x.as_ref()).collect();
    vec![
        tile[0].to_string(),
        tile[tile.len() - 1].to_string(),
        extract_col(&tile[..], 0),
        extract_col(&tile[..], tile.len() - 1),
    ]
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn build_tuple(s1: String, s2: String) -> (String, String) {
    if s1 < s2 {
        (s1, s2)
    } else {
        (s2, s1)
    }
}

fn extract_col<T: AsRef<str>>(tile: &[T], col: usize) -> String {
    tile.iter()
        .map(|x| x.as_ref().chars().nth(col).unwrap())
        .collect()
}

fn flip(tile: &[String]) -> Vec<String> {
    let mut updated: Vec<String> = Vec::new();
    for row in tile.iter().rev() {
        updated.push(row.to_string());
    }
    updated
}

fn rotate(tile: &[String]) -> Vec<String> {
    let mut updated = Vec::new();
    for i in 0..tile.len() {
        updated.push(reverse(&extract_col(tile, i)));
    }
    updated
}

fn generate_possible_image_combs(image: &[String]) -> Vec<Vec<String>> {
    let mut image: Vec<String> = image.iter().map(|x| x.to_string()).collect();
    let mut i = 0;
    let mut out = Vec::new();

    while i < 4 {
        out.push(image.clone());
        image = rotate(&image[..]);
        i += 1;
    }

    image = flip(&image[..]);
    i = 0;
    while i < 4 {
        out.push(image.clone());
        image = rotate(&image[..]);
        i += 1;
    }

    out
}

fn check_constraints(tile: &[String], constraints: &[(Edge, String)]) -> bool {
    for (e, s) in constraints {
        let other = match e {
            Edge::Left => extract_col(tile, 0),
            Edge::Up => tile[0].clone(),
        };

        if s != &other {
            return false;
        }
    }
    true
}

fn arrange_tile(tile: &[String], constraints: &[(Edge, String)]) -> Option<Vec<String>> {
    let combs = generate_possible_image_combs(tile);

    for comb in combs {
        if check_constraints(&comb[..], constraints) {
            return Some(comb);
        }
    }

    None
}

fn is_unconnected_edge(e: &str, h: &HashMap<(String, String), Vec<usize>>) -> bool {
    let e = e.to_string();
    let re = reverse(&e);
    h[&build_tuple(e, re)].len() == 1
}

fn get_corner_edges(tile: &[String], h: &HashMap<(String, String), Vec<usize>>) -> Vec<String> {
    let tile_length = tile.len();
    let up = is_unconnected_edge(&tile[0], h);
    let left = is_unconnected_edge(&extract_col(tile, 0), h);
    let down = is_unconnected_edge(&tile[tile_length - 1], h);
    let right = is_unconnected_edge(&extract_col(tile, tile_length - 1), h);

    if left && down {
        vec![
            tile[tile_length - 1].clone(),
            reverse(&extract_col(tile, 0)),
        ]
    } else if down && right {
        vec![
            reverse(&extract_col(tile, tile_length - 1)),
            reverse(&tile[tile_length - 1]),
        ]
    } else if right && up {
        vec![reverse(&tile[0]), extract_col(tile, tile_length - 1)]
    } else if up && left {
        vec![extract_col(tile, 0), tile[0].clone()]
    } else {
        panic!("Not a corner");
    }
}

fn get_possible_tile(
    tiles: &[Vec<String>],
    constraints: &[(Edge, String)],
    chosen: &mut HashSet<usize>,
) -> Vec<String> {
    let mut v: Vec<Vec<String>> = Vec::new();

    for (i, tile) in tiles.iter().enumerate() {
        if chosen.contains(&i) {
            continue;
        }

        if let Some(ans) = arrange_tile(&tile[..], constraints) {
            chosen.insert(i);
            v.push(ans);
        }
    }

    assert!(v.len() == 1);
    v.pop().unwrap()
}

fn remove_border(tile: Vec<String>) -> Vec<String> {
    let l = tile.len();
    tile[1..l - 1]
        .iter()
        .map(|x| x[1..l - 1].to_string())
        .collect()
}

fn build_image(
    corner_tile: usize,
    tiles: &[Vec<&str>],
    h: &HashMap<(String, String), Vec<usize>>,
) -> Vec<String> {
    let tiles: Vec<Vec<String>> = tiles
        .iter()
        .map(|tile| {
            tile.iter()
                .map(|row| row.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    let length: usize = (tiles.len() as f32).sqrt() as usize;
    let tile_length = tiles[0].len();

    let mut arrange: Vec<Vec<Vec<String>>> = vec![vec![vec![String::new(); 4]; length]; length];
    let mut chosen: HashSet<usize> = HashSet::new();

    for r in 0..length {
        for c in 0..length {
            let mut constraints = Vec::new();

            if r == 0 && c == 0 {
                let corner_edges = get_corner_edges(&tiles[corner_tile], h);
                constraints.push((Edge::Left, corner_edges[0].clone()));
                constraints.push((Edge::Up, corner_edges[1].clone()));
            } else if c != 0 {
                constraints.push((Edge::Left, extract_col(&arrange[r][c - 1], tile_length - 1)));
            } else if r != 0 {
                constraints.push((Edge::Up, arrange[r - 1][c][tile_length - 1].clone()));
            }

            arrange[r][c] = get_possible_tile(&tiles[..], &constraints, &mut chosen);
        }
    }

    // Remove border of the tiles
    let arrange: Vec<Vec<Vec<String>>> = arrange
        .into_iter()
        .map(|row| row.into_iter().map(remove_border).collect())
        .collect();

    let mut out = Vec::new();
    for row in arrange {
        let mut this = vec![String::new(); tile_length];
        for tile in row {
            let mut update = Vec::new();
            for i in 0..tile.len() {
                update.push(this[i].clone() + &tile[i]);
            }
            this = update;
        }

        out.append(&mut this);
    }
    out
}

fn matches_pattern<T: AsRef<str>>(pattern: &[T], image: &[T], r: usize, c: usize) -> bool {
    let image: Vec<Vec<char>> = image
        .iter()
        .map(|x| x.as_ref().chars().collect::<Vec<char>>())
        .collect();
    let pattern: Vec<Vec<char>> = pattern
        .iter()
        .map(|x| x.as_ref().chars().collect::<Vec<char>>())
        .collect();

    if pattern.len() + r >= image.len() {
        return false;
    }

    if pattern[0].len() + c >= image[0].len() {
        return false;
    }

    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            if pattern[i][j] == '#' && image[r + i][c + j] != '#' {
                return false;
            }
        }
    }

    true
}

fn count_patterns<T: AsRef<str>>(pattern: &[T], image: &[T]) -> u32 {
    let mut ans = 0;
    for r in 0..image.len() {
        for c in 0..image[0].as_ref().len() {
            if matches_pattern(pattern, image, r, c) {
                ans += 1;
            }
        }
    }
    ans
}

fn count_hash(image: &[String]) -> u32 {
    image
        .iter()
        .map(|x| x.chars().map(|x| (x == '#') as u32).sum::<u32>())
        .sum()
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

    let mut h: HashMap<(String, String), Vec<usize>> = HashMap::new();

    for (i, tile) in tiles.iter().enumerate() {
        for edge in edges(tile) {
            let re = reverse(&edge);
            let x = h.entry(build_tuple(edge, re)).or_insert_with(Vec::new);
            x.push(i);
        }
    }

    let mut corners = Vec::new();
    for (i, tile) in tiles.iter().enumerate() {
        let mut count: u8 = 0;
        for edge in edges(tile) {
            let re = reverse(&edge);
            count += (h[&build_tuple(edge, re)].len() == 1) as u8;
        }

        if count == 2 {
            corners.push(i);
        }
    }

    let image = build_image(corners[0], &tiles, &h);
    let pattern = vec![
        "                  # ".to_string(),
        "#    ##    ##    ###".to_string(),
        " #  #  #  #  #  #   ".to_string(),
    ];

    let mut max = 0;
    for image in generate_possible_image_combs(&image[..]) {
        let this = count_patterns(&pattern[..], &image[..]);
        if this > max {
            max = this;
        }
    }

    println!(
        "{}",
        count_hash(&image[..]) - max * count_hash(&pattern[..])
    );
}
