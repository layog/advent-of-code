use std::io::Read;

const SUBJECT: u64 = 7;
const M: u64 = 20201227;

fn get_n(remainder: u64) -> u64 {
    let mut n: u64 = 0;
    while modpow(SUBJECT, n, M) != remainder {
        n += 1;
    }
    n
}

fn modpow(base: u64, exp: u64, m: u64) -> u64 {
    if exp == 0 {
        return 1;
    }

    if exp % 2 == 1 {
        (base * modpow(base, exp - 1, m)) % m
    } else {
        modpow((base * base) % m, exp / 2, m)
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let v: Vec<_> = buffer.trim().split('\n').collect();
    assert!(v.len() == 2);

    let one: u64 = v[0].parse().unwrap();
    let two: u64 = v[1].parse().unwrap();

    let n1 = get_n(one);

    println!("{}", modpow(two, n1, M));
}
