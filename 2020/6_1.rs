use std::io::Read;
use std::collections::HashSet;


fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut count = 0;
    for group in buffer.split("\n\n") {
        let mut s = HashSet::new();
        for person in group.split('\n') {
            for c in person.trim().chars() {
                s.insert(c);
            }
        }

        count += s.len();
    }
    println!("{}", count);

    Ok(())
}
