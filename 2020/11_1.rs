use std::io::Read;

fn in_bounds(x: isize, max: isize) -> bool {
    x >= 0 && x < max
}

fn count(seats: &[Vec<char>], r: usize, c: usize) -> u8 {
    const DELTAS: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    if seats[r][c] == '.' {
        return 0;
    }

    let mut ans: u8 = 0;
    for &(ri, ci) in DELTAS.iter() {
        let nr: isize;
        let nc: isize;
        nr = r as isize + ri as isize;
        nc = c as isize + ci as isize;

        if in_bounds(nr, seats.len() as isize) && in_bounds(nc, seats[0].len() as isize) {
            ans += (seats[nr as usize][nc as usize] == '#') as u8;
        }
    }
    ans
}

fn perform(seats: &mut Vec<Vec<char>>) -> bool {
    let rows = seats.len();
    let cols = seats[0].len();

    let mut seat_count: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    for (r, count_row) in seat_count.iter_mut().enumerate().take(rows) {
        for (c, cc) in count_row.iter_mut().enumerate().take(cols) {
            *cc = count(&seats, r, c);
        }
    }

    let mut change = false;

    for (r, count_row) in seat_count.iter().enumerate().take(rows) {
        for (c, &count) in count_row.iter().enumerate().take(cols) {
            if seats[r][c] == '.' {
                continue;
            }

            if count >= 4 && seats[r][c] == '#' {
                seats[r][c] = 'L';
                change = true;
            }

            if count == 0 && seats[r][c] == 'L' {
                seats[r][c] = '#';
                change = true;
            }
        }
    }
    change
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let mut seats: Vec<Vec<char>> = buffer
        .trim()
        .split('\n')
        .map(|x| x.trim().chars().collect::<Vec<char>>())
        .collect();

    while perform(&mut seats) {}

    println!(
        "{}",
        seats
            .into_iter()
            .map(|x| x.into_iter().filter(|&x| x == '#').count())
            .sum::<usize>()
    );

    Ok(())
}
