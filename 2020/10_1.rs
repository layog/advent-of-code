use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let mut nums: Vec<i32> = buffer
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    nums.sort();

    let mut x: i32 = 0;
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 1;
    for n in nums {
        let v = match n - x {
            1 => &mut count1,
            2 => &mut count2,
            3 => &mut count3,
            _ => { panic!("Unknown"); },
        };

        *v += 1;
        x = n;
    }
    println!("{}", count1 * count3);

    Ok(())
}
