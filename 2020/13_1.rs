use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let lines: Vec<&str> = buffer.trim().split('\n').map(|x| x.trim()).collect();
    assert!(lines.len() == 2);

    let arrival: u32 = lines[0].parse().unwrap();
    let bus_times: Vec<u32> = lines[1]
        .split(',')
        .filter(|x| x.trim() != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut wait_time: u32 = *bus_times.iter().max().unwrap();
    let mut bus: u32 = 0;

    for time in bus_times {
        if (time - (arrival % time)) <= wait_time {
            wait_time = time - (arrival % time);
            bus = time;
        }
    }

    println!("{}", wait_time * bus);

    Ok(())
}
