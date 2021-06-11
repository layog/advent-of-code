use std::collections::{HashMap, HashSet};
use std::io::Read;

struct Range {
    start: u32,
    end: u32,
}

fn to_range(s: &str) -> Vec<Range> {
    let s: Vec<&str> = s.split(':').collect();

    let mut out = Vec::new();
    for range in s[1].trim().split("or") {
        let v: Vec<u32> = range
            .trim()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        assert!(v.len() == 2);
        out.push(Range {
            start: v[0],
            end: v[1],
        });
    }

    out
}

fn filter_valid_tickets(tickets: &str, fields: &[Vec<Range>]) -> Vec<Vec<u32>> {
    let mut valid_tickets = Vec::new();
    for s in tickets.trim().split('\n').skip(1) {
        let nums: Vec<u32> = s.trim().split(',').map(|x| x.parse().unwrap()).collect();
        let mut valid = true;

        for &n in nums.iter() {
            let mut found_match = false;
            for field in fields.iter() {
                if contains(n, field) {
                    found_match = true;
                    break;
                }
            }

            if !found_match {
                valid = false;
                break;
            }
        }

        if valid {
            valid_tickets.push(nums);
        }
    }

    valid_tickets
}

fn invert(matrix: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut inv: Vec<Vec<u32>> = vec![vec![0; rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            inv[c][r] = matrix[r][c];
        }
    }
    inv
}

fn contains(n: u32, field: &[Range]) -> bool {
    for range in field.iter() {
        if n >= range.start && n <= range.end {
            return true;
        }
    }
    false
}

fn check(to_check: &[u32], field: &[Range]) -> bool {
    for n in to_check {
        if !contains(*n, field) {
            return false;
        }
    }
    true
}

fn assign_index(fields: &[Vec<Range>], tickets: &[Vec<u32>]) -> Vec<usize> {
    let m = invert(tickets);
    assert!(m.len() == fields.len());

    let mut possible: Vec<HashSet<usize>> = Vec::new();
    for to_check in m {
        let mut this = HashSet::new();
        for (idx, field) in fields.iter().enumerate() {
            if check(&to_check, field) {
                this.insert(idx);
            }
        }
        possible.push(this);
    }

    let mut done: HashSet<usize> = HashSet::new();
    while done.len() < fields.len() {
        for hs in possible.iter_mut() {
            for n in done.iter() {
                if hs.len() == 1 {
                    break;
                }

                hs.remove(n);
            }

            if hs.len() == 1 {
                done.insert(*hs.iter().next().unwrap());
            }
        }
    }

    possible
        .iter()
        .map(|hs| *(hs.iter().next().unwrap()))
        .collect()
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let v: Vec<&str> = buffer.trim().split("\n\n").collect();
    assert!(v.len() == 3);

    let fields: Vec<Vec<Range>> = v[0].split('\n').map(|x| to_range(x.trim())).collect();
    let tickets = filter_valid_tickets(v[2], &fields);
    let ticket: Vec<u32> = v[1].trim().split('\n').collect::<Vec<&str>>()[1]
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let indices: HashMap<usize, usize> = assign_index(&fields, &tickets)
        .iter()
        .enumerate()
        .map(|x| (*x.1, x.0))
        .collect();

    let starting_with_departure: Vec<usize> = v[0]
        .trim()
        .split('\n')
        .enumerate()
        .map(|x| (x.0, x.1.split(':').next().unwrap().trim()))
        .filter(|x| x.1.starts_with("departure"))
        .map(|x| x.0)
        .collect();

    let mut ans: u128 = 1;
    for idx in starting_with_departure {
        ans *= ticket[indices[&idx]] as u128;
    }
    println!("{}", ans);

    Ok(())
}
