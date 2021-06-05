use std::collections::HashSet;
use std::io::Read;

fn calculate_id(seat: &str) -> i32 {
    let row = &seat[..7];
    let col = &seat[7..];

    let mut r: i32 = 0;
    for c in row.chars() {
        let x: i32 = (c == 'B') as i32;
        r = (r * 2) + x;
    }

    let mut cc: i32 = 0;
    for c in col.chars() {
        let x: i32 = (c == 'R') as i32;
        cc = (cc * 2) + x;
    }
    r * 8 + cc
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut min: i32 = 127 * 8 + 7;
    let mut max: i32 = 0;
    let mut found = HashSet::new();

    for seat in buffer.split('\n').filter(|x| x.len() == 10) {
        let tid = calculate_id(seat);
        found.insert(tid);

        if tid > max {
            max = tid;
        }

        if tid < min {
            min = tid;
        }
    }

    for i in min..max {
        if !found.contains(&i) {
            println!("{}", i);
            break;
        }
    }

    Ok(())
}
