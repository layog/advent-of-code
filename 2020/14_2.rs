use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
enum Command {
    Mask(String),
    Mem(u64, u64),
}

fn generate(l: u64, mask: &[char], idx: usize) -> Vec<u64> {
    if idx == mask.len() {
        return vec![l];
    }

    let bit: u64 = (mask.len() - idx - 1) as u64;

    if mask[idx] == '0' {
        return generate(l, mask, idx + 1);
    }

    if mask[idx] == '1' {
        return generate(l | (1 << bit), mask, idx + 1);
    }

    let and: u64 = !(1 << bit);
    let or: u64 = 1 << bit;

    let mut out: Vec<u64> = Vec::new();
    for loc in generate(l, mask, idx + 1) {
        out.push(loc & and);
        out.push(loc | or);
    }
    out
}

fn parse_to_command(s: &str) -> Command {
    let v: Vec<&str> = s.split('=').map(|x| x.trim()).collect();
    assert!(v.len() == 2);

    if v[0].starts_with("mask") {
        Command::Mask(v[1].to_string())
    } else {
        Command::Mem(
            v[0][4..v[0].len() - 1].parse().unwrap(),
            v[1].parse().unwrap(),
        )
    }
}

fn update_mem(m: &mut HashMap<u64, u64>, l: u64, v: u64) {
    m.insert(l, v);
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let commands: Vec<Command> = buffer
        .trim()
        .split('\n')
        .map(|x| parse_to_command(x.trim()))
        .collect();

    let mut m: HashMap<u64, u64> = HashMap::new();
    let mut mask: Vec<char> = Vec::new();

    for command in commands {
        match command {
            Command::Mask(x) => {
                mask = x.chars().collect();
            }
            Command::Mem(l, v) => {
                for loc in generate(l, &mask, 0) {
                    update_mem(&mut m, loc, v);
                }
            }
        }
    }

    println!("{}", m.values().sum::<u64>());

    Ok(())
}
