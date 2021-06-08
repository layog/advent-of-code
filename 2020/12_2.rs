use std::io::Read;

fn rotate(pos: &mut (i32, i32), mut deg: i32) {
    while deg != 0 {
        let &mut (x, y) = pos;
        *pos = (-y, x);
        deg -= 90;
    }
}

fn movement(waypoint: &mut (i32, i32), c: char, v: i32) -> (i32, i32) {
    if c == 'L' {
        rotate(waypoint, v);
        return (0, 0);
    }
    if c == 'R' {
        rotate(waypoint, (-v + 360) % 360);
        return (0, 0);
    }

    if c == 'F' {
        return (v * waypoint.0, v * waypoint.1);
    }

    let update = match c {
        'N' => (0, v),
        'S' => (0, -v),
        'E' => (v, 0),
        'W' => (-v, 0),
        _ => {
            panic!("Unknown c: {}", c);
        }
    };

    waypoint.0 += update.0;
    waypoint.1 += update.1;

    (0, 0)
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let instructions: Vec<(char, i32)> = buffer
        .trim()
        .split('\n')
        .map(|x| x.trim())
        .map(|x| (x.chars().next().unwrap(), x[1..].parse().unwrap()))
        .collect();

    let mut waypoint: (i32, i32) = (10, 1);
    let mut pos: (i32, i32) = (0, 0);

    for &(c, v) in instructions.iter() {
        let x = movement(&mut waypoint, c, v);
        pos.0 += x.0;
        pos.1 += x.1;
    }

    println!("{}", pos.0.abs() + pos.1.abs());

    Ok(())
}
