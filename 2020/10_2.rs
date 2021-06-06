use std::collections::HashMap;
use std::io::Read;

fn helper(nums: &[i32], idx: usize, done: &mut HashMap<usize, i64>) -> i64 {
    if idx == nums.len() - 1 {
        return 1;
    }

    if !done.contains_key(&idx) {
        let mut next = idx + 1;
        let mut ans = 0;
        while next < nums.len() && ((nums[next] - nums[idx]) <= 3) {
            ans += helper(nums, next, done);
            next += 1;
        }
        done.insert(idx, ans);
    }

    *done.get(&idx).unwrap()
}

fn find_combinations(nums: &[i32]) -> i64 {
    let mut done = HashMap::new();
    helper(nums, 0, &mut done)
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let mut nums: Vec<i32> = buffer
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    nums.push(0);
    nums.sort_unstable();

    println!("{}", find_combinations(&nums));

    Ok(())
}
