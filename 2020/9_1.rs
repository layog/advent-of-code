use std::collections::HashSet;
use std::io::Read;

fn check(nums: &[i128], val: i128) -> bool {
    let mut h: HashSet<i128> = HashSet::new();

    for n in nums {
        let x = val - n;
        if h.contains(&x) {
            return false;
        }
        h.insert(*n);
    }
    true
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let nums: Vec<i128> = buffer
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    let preamble = 25;
    for i in preamble..nums.len() {
        if check(&nums[i - preamble..i], nums[i]) {
            println!("{}", nums[i]);
            break;
        }
    }

    Ok(())
}
