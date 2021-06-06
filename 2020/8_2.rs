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

fn parse(line: &str) -> (String, i32) {
    let v: Vec<&str> = line.split(' ').map(|x| x.trim()).collect();
    assert!(v.len() == 2);

    let val: i32 = v[1].parse().unwrap();
    (v[0].trim().to_string(), val)
}

fn check(instructions: &[(String, i32)]) -> (bool, i32) {
    let mut index: usize = 0;
    let mut seen: HashSet<usize> = HashSet::new();
    let mut answer: i32 = 0;

    loop {
        if seen.contains(&index) || (index > instructions.len()) {
            break;
        }
        seen.insert(index);
        let (f, v) = &instructions[index];
        let f: FnType = match f as &str {
            "nop" => nop,
            "jmp" => jmp,
            "acc" => acc,
            _ => {
                panic!("Unknown");
            }
        };
        answer += f(&mut index, *v);

        if index == instructions.len() {
            return (true, answer);
        }
    }

    (false, 0)
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut instructions: Vec<(String, i32)> =
        buffer.trim().split('\n').map(|x| parse(x)).collect();

    let mut acc: i32 = 0;
    for i in 0..instructions.len() {
        let current: String = instructions[i].0.to_string();
        let replace: &str = match &current as &str {
            x if x == "jmp" => "nop",
            x if x == "nop" => "jmp",
            _ => continue,
        };

        instructions[i] = (replace.to_string(), instructions[i].1);
        let (x, v) = check(&instructions);
        if x {
            acc = v;
            break;
        }
        instructions[i] = (current, instructions[i].1);
    }

    println!("{}", acc);

    Ok(())
}
