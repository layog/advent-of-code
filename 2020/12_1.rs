use std::io::Read;

fn sin(x: i32) -> i32 {
    match x {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => {
            panic!("Unknown value");
        }
    }
}

fn cos(x: i32) -> i32 {
    match x {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => {
            panic!("Unknown value");
        }
    }
}

fn movement(direction: &mut i32, c: char, v: i32) -> (i32, i32) {
    if c == 'L' {
        *direction = (*direction + v) % 360;
        return (0, 0);
    }
    if c == 'R' {
        *direction = (*direction - v + 360) % 360;
        return (0, 0);
    }

    if c == 'F' {
        return (v * cos(*direction), v * sin(*direction));
    }

    match c {
        'N' => (0, v),
        'S' => (0, -v),
        'E' => (v, 0),
        'W' => (-v, 0),
        _ => {
            panic!("Unknown c: {}", c);
        }
    }
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

    let mut direction: i32 = 0;
    let mut pos: (i32, i32) = (0, 0);

    for &(c, v) in instructions.iter() {
        let x = movement(&mut direction, c, v);
        pos.0 += x.0;
        pos.1 += x.1;
    }

    println!("{}", pos.0.abs() + pos.1.abs());

    Ok(())
}
