use std::collections::HashSet;
use std::io::Read;

type FnType = fn(&mut usize, i32) -> i32;

fn nop(index: &mut usize, _: i32) -> i32 {
    *index += 1;
    0
}

fn jmp(index: &mut usize, v: i32) -> i32 {
    *index = ((*index as i32) + v) as usize;
    0
}

fn acc(index: &mut usize, v: i32) -> i32 {
    *index += 1;
    v
}

fn parse(line: &str) -> (FnType, i32) {
    let v: Vec<&str> = line.split(' ').map(|x| x.trim()).collect();
    assert!(v.len() == 2);

    let val: i32 = v[1].parse().unwrap();
    let f = match v[0] {
        "jmp" => jmp,
        "nop" => nop,
        "acc" => acc,
        _ => {
            panic!("Unknown instruction: {}", v[0]);
        }
    };
    (f, val)
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let instructions: Vec<(FnType, i32)> =
        buffer.trim().split('\n').map(|x| parse(x)).collect();
    let mut index: usize = 0;

    let mut seen: HashSet<usize> = HashSet::new();
    let mut acc: i32 = 0;
    loop {
        if seen.contains(&index) {
            break;
        }
        seen.insert(index);
        let (f, v) = instructions[index];
        acc += f(&mut index, v);
    }

    println!("{}", acc);

    Ok(())
}
