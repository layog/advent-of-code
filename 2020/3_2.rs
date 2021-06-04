use std::io::Read;

fn get_trees(area: &[Vec<char>], slope: (usize, usize)) -> usize {
    let (rows, cols) = (area.len(), area[0].len());
    let mut pos = (0, 0);
    let mut count = 0;

    while pos.0 < rows {
        count += (area[pos.0][pos.1] == '#') as usize;

        pos.0 += slope.0;
        pos.1 += slope.1;
        pos.1 %= cols;
    }

    count
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let area: Vec<Vec<char>> = buffer
        .trim()
        .split('\n')
        .map(|x| x.trim().chars().collect())
        .collect();

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut answer = 1;
    for slope in slopes {
        answer *= get_trees(&area, slope);
    }
    println!("{}", answer);
    Ok(())
}
