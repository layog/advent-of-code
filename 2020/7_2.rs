use std::collections::HashMap;
use std::io::Read;

type BagVec = Vec<(i64, String)>;

fn parse_bag(line: &str) -> String {
    line.trim()
        .trim_end_matches('.')
        .trim_end_matches(" bag")
        .trim_end_matches(" bags")
        .to_string()
}

fn parse_bags_list(line: &str) -> BagVec {
    let v: Vec<&str> = line
        .split(',')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let mut out: BagVec = Vec::new();

    for bag in v {
        let x: Vec<&str> = bag.splitn(2, ' ').collect();
        assert!(x.len() == 2);

        if let Ok(y) = x[0].parse() {
            out.push((y, parse_bag(x[1])));
        }
    }
    out
}

fn parse_line(line: &str) -> (String, BagVec) {
    let v: Vec<&str> = line.split("contain").collect();
    assert!(v.len() == 2);
    (parse_bag(v[0]), parse_bags_list(v[1].trim()))
}

fn get_count(h: &HashMap<String, BagVec>, start: &str) -> i64 {
    let mut ans = 0;
    for next in h.get(start).unwrap_or(&Vec::new()) {
        ans += (1 + get_count(h, &next.1)) * next.0;
    }
    ans
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut bag_map = HashMap::new();
    for line in buffer.trim().split('\n') {
        let (bag, mut contains) = parse_line(&line);
        let x = bag_map.entry(bag).or_insert_with(Vec::new);
        x.append(&mut contains);
    }

    println!("{}", get_count(&bag_map, "shiny gold"));

    Ok(())
}
