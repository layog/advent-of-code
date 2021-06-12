use std::io::Read;

type CellType = (usize, usize, usize, usize);
type StateType = Vec<Vec<Vec<Vec<bool>>>>;

fn get_nearby(cell: &CellType, limits: &CellType) -> Vec<CellType> {
    let mut cells: Vec<CellType> = Vec::new();
    let deltas: [isize; 3] = [-1, 0, 1];
    let &(w, i, j, k) = cell;

    let &(lw, li, lj, lk) = limits;
    let lw = lw as isize;
    let li = li as isize;
    let lj = lj as isize;
    let lk = lk as isize;

    for &dw in deltas.iter() {
        for &di in deltas.iter() {
            for &dj in deltas.iter() {
                for &dk in deltas.iter() {
                    if di != 0 || dj != 0 || dk != 0 || dw != 0 {
                        let w = (w as isize) + dw;
                        let i = (i as isize) + di;
                        let j = (j as isize) + dj;
                        let k = (k as isize) + dk;

                        if w >= 0
                            && i >= 0
                            && j >= 0
                            && k >= 0
                            && w < lw
                            && i < li
                            && j < lj
                            && k < lk
                        {
                            cells.push((w as usize, i as usize, j as usize, k as usize));
                        }
                    }
                }
            }
        }
    }

    cells
}

fn get_activation(state: &StateType, cell: &CellType) -> bool {
    let mut nearby: u8 = 0;
    let cells = get_nearby(
        cell,
        &(
            state.len(),
            state[0].len(),
            state[0][0].len(),
            state[0][0][0].len(),
        ),
    );

    for (cw, ci, cj, ck) in cells {
        nearby += state[cw][ci][cj][ck] as u8;
    }

    let &(w, i, j, k) = cell;

    if state[w][i][j][k] {
        (2..=3_u8).contains(&nearby)
    } else {
        nearby == 3
    }
}

fn perform(state: StateType) -> StateType {
    let mut updated: StateType = state.clone();

    for w in 0..state.len() {
        for i in 0..state[0].len() {
            for j in 0..state[0][0].len() {
                for k in 0..state[0][0][0].len() {
                    updated[w][i][j][k] = get_activation(&state, &(w, i, j, k));
                }
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

    // Order -> w(0) - z(1) - y(2) - x(3)
    let mut state: StateType = Vec::new();
    for _ in 0..(cycles * 2 + 1) {
        let mut this = Vec::new();
        for _ in 0..(cycles * 2 + 1) {
            this.push(vec![
                vec![false; cycles * 2 + v[0].len()];
                cycles * 2 + v.len()
            ]);
        }
        state.push(this);
    }

    for (i, row) in v.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            state[cycles][cycles][cycles + i][cycles + j] = c == '#';
        }
    }

    for _ in 0..cycles {
        state = perform(state);
    }

    let ans: u16 = state
        .into_iter()
        .map(|dimension| {
            dimension
                .into_iter()
                .map(|field| {
                    field
                        .into_iter()
                        .map(|row| row.into_iter().filter(|&x| x).count() as u16)
                        .sum::<u16>()
                })
                .sum::<u16>()
        })
        .sum();
    println!("{}", ans);
}
