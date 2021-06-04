use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let nums: Vec<i32> = buffer
        .trim_end()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    let l = nums.len();
    for i in 0..l {
        for j in i + 1..l {
            for k in j + 1..l {
                if (nums[i] + nums[j] + nums[k]) == 2020 {
                    println!("{}", nums[i] * nums[j] * nums[k]);
                }
            }
        }
    }
    Ok(())
}
