use std::collections::HashMap;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let nums: Vec<u32> = buffer
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut m: HashMap<u32, u32> = HashMap::new();
    let mut count: u32 = 1;
    let mut prev: u32 = 0;

    for num in nums {
        let x = m.entry(num).or_insert(count);
        prev = count - *x;
        *x = count;
        count += 1;
    }

    while count < 30000000 {
        let x = m.entry(prev).or_insert(count);
        prev = count - *x;
        *x = count;
        count += 1;
    }

    println!("{}", prev);

    Ok(())
}
