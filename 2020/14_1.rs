use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
enum Command {
    Mask(String),
    Mem(u64, u64),
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

fn update_masks(and: &mut u64, or: &mut u64, mask: String) {
    *and = !0;
    *or = 0;

    for (i, c) in mask.chars().rev().enumerate() {
        if c == '0' {
            *and &= !(1 << i);
        } else if c == '1' {
            *or |= 1 << i;
        }
    }
}

fn update_mem(m: &mut HashMap<u64, u128>, l: u64, v: u64, and: u64, or: u64) {
    m.insert(l, ((v & and) | or).into());
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let commands: Vec<Command> = buffer
        .trim()
        .split('\n')
        .map(|x| parse_to_command(x.trim()))
        .collect();

    let mut m: HashMap<u64, u128> = HashMap::new();
    let mut and = !0;
    let mut or = 0;

    for command in commands {
        match command {
            Command::Mask(x) => update_masks(&mut and, &mut or, x),
            Command::Mem(l, v) => update_mem(&mut m, l, v, and, or),
        }
    }

    println!("{}", m.values().sum::<u128>());

    Ok(())
}
