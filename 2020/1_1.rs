use std::collections::HashSet;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let nums: Vec<i32> = buffer
        .trim_end()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut seen = HashSet::new();

    for num in nums {
        let x = 2020 - num;
        if seen.contains(&x) {
            println!("{}", x * num);
            break;
        }
        seen.insert(num);
    }
    Ok(())
}
