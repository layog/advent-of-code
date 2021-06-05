use std::collections::HashSet;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut count = 0;
    for group in buffer.split("\n\n") {
        let mut s = HashSet::new();
        for c in 'a'..='z' {
            s.insert(c);
        }

        for person in group.split('\n').filter(|x| !x.trim().is_empty()) {
            let mut this = HashSet::new();
            for c in person.trim().chars() {
                this.insert(c);
            }
            s = s.intersection(&this).cloned().collect();
        }

        count += s.len();
    }
    println!("{}", count);

    Ok(())
}
