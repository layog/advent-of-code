// It's Chinese Remainder Theorem, copying from -
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

use std::io::Read;

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let lines: Vec<&str> = buffer.trim().split('\n').map(|x| x.trim()).collect();
    assert!(lines.len() == 2);

    let bus_times: Vec<(usize, usize)> = lines[1]
        .split(',')
        .enumerate()
        .filter(|x| x.1.trim() != "x")
        .map(|x| (x.0, x.1.parse().unwrap()))
        .collect();

    let mut modulii = Vec::new();
    let mut residues = Vec::new();

    for (idx, v) in bus_times {
        modulii.push(v as i64);
        let r = (v - (idx % v)) % v;
        residues.push(r as i64);
    }

    match chinese_remainder(&residues, &modulii) {
        Some(sol) => println!("{}", sol),
        None => println!("modulii not pairwise coprime"),
    }

    Ok(())
}
