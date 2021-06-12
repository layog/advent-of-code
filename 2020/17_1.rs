use std::io::Read;

type CellType = (usize, usize, usize);
type StateType = Vec<Vec<Vec<bool>>>;

fn get_nearby(cell: &CellType, limits: &CellType) -> Vec<CellType> {
    let mut cells: Vec<CellType> = Vec::new();
    let deltas: [isize; 3] = [-1, 0, 1];
    let &(i, j, k) = cell;

    let &(li, lj, lk) = limits;
    let li = li as isize;
    let lj = lj as isize;
    let lk = lk as isize;

    for &di in deltas.iter() {
        for &dj in deltas.iter() {
            for &dk in deltas.iter() {
                if di != 0 || dj != 0 || dk != 0 {
                    let i = (i as isize) + di;
                    let j = (j as isize) + dj;
                    let k = (k as isize) + dk;

                    if i >= 0 && j >= 0 && k >= 0 &&
                       i < li && j < lj && k < lk {
                        cells.push((i as usize, j as usize, k as usize));
                    }
                }
            }
        }
    }

    cells
}

fn get_activation(state: &StateType, cell: &CellType) -> bool {
    let mut nearby: u8 = 0;
    let cells = get_nearby(cell, &(state.len(), state[0].len(), state[0][0].len()));

    for (ci, cj, ck) in cells {
        nearby += state[ci][cj][ck] as u8;
    }

    let &(i, j, k) = cell;

    if state[i][j][k] {
        (2..=3_u8).contains(&nearby)
    } else {
        nearby == 3
    }
}

fn perform(state: StateType) -> StateType {
    let mut updated: StateType = state.clone();

    for i in 0..state.len() {
        for j in 0..state[0].len() {
            for k in 0..state[0][0].len() {
                updated[i][j][k] = get_activation(&state, &(i, j, k));
            }
        }
    }

    updated
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let cycles = 6;

    let v: Vec<Vec<char>> = buffer
        .trim()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    // Order -> z(0) - y(1) - x(2)
    let mut state: StateType = Vec::new();
    for _ in 0..(cycles * 2 + 1) {
        state.push(vec![
            vec![false; cycles * 2 + v[0].len()];
            cycles * 2 + v.len()
        ]);
    }

    for (i, row) in v.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            state[cycles][cycles + i][cycles + j] = c == '#';
        }
    }

    for _ in 0..cycles {
        state = perform(state);
    }

    let ans: u16 = state
        .into_iter()
        .map(|field| {
            field
                .into_iter()
                .map(|row| row.into_iter().filter(|&x| x).count() as u16)
                .sum::<u16>()
        })
        .sum();
    println!("{}", ans);
}
