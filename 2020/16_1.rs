use std::io::Read;

struct Range {
    start: u32,
    end: u32,
}

fn to_range(s: &str) -> Vec<Range> {
    let s: Vec<&str> = s.split(':').collect();

    let mut out = Vec::new();
    for range in s[1].trim().split("or") {
        let v: Vec<u32> = range
            .trim()
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect();
        assert!(v.len() == 2);
        out.push(Range {
            start: v[0],
            end: v[1],
        });
    }

    out
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let v: Vec<&str> = buffer.trim().split("\n\n").collect();
    assert!(v.len() == 3);

    let fields: Vec<Vec<Range>> = v[0].split('\n').map(|x| to_range(x.trim())).collect();

    let mut ans = 0;

    for s in v[2].trim().split('\n').skip(1) {
        let nums: Vec<u32> = s.trim().split(',').map(|x| x.parse().unwrap()).collect();

        for n in nums {
            let mut found_match = false;
            for field in fields.iter() {
                for range in field.iter() {
                    if n >= range.start && n <= range.end {
                        found_match = true;
                        break;
                    }
                }

                if found_match {
                    break;
                }
            }

            if !found_match {
                ans += n;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
