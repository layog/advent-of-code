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

    let mut id: i32 = 0;
    for seat in buffer.split('\n').filter(|x| x.len() == 10) {
        let tid = calculate_id(seat);
        if tid > id {
            id = tid;
        }
    }
    println!("{}", id);

    Ok(())
}
