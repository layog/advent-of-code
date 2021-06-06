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

fn find_set(nums: &[i128], val: i128) -> &[i128] {
    let mut width = 0;
    let mut sum = 0;

    let mut start = 0;
    let mut end = 0;

    while end < nums.len() {
        if sum < val {
            sum += nums[end];
            end += 1;
            width += 1;
        } else {
            sum -= nums[start];
            start += 1;
            width -= 1;
        }

        if (sum == val) && width > 1 {
            return &nums[start..end];
        }
    }
    panic!("Shouldn't reach here");
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
    let mut val = 0;
    for i in preamble..nums.len() {
        if check(&nums[i - preamble..i], nums[i]) {
            val = nums[i];
            break;
        }
    }

    let x = find_set(&nums, val);
    println!("{}", x.iter().min().unwrap() + x.iter().max().unwrap());

    Ok(())
}
