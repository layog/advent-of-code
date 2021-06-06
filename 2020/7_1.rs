use std::collections::{HashMap, HashSet};
use std::io::Read;

type BagVec = Vec<String>;

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

        if x[0].parse::<i32>().is_ok() {
            out.push(parse_bag(x[1]));
        }
    }
    out
}

fn parse_line(line: &str) -> (String, BagVec) {
    let v: Vec<&str> = line.split("contain").collect();
    assert!(v.len() == 2);
    (parse_bag(v[0]), parse_bags_list(v[1].trim()))
}

fn get_count_helper(h: &HashMap<String, BagVec>, start: &str, seen: &mut HashSet<String>) -> i32 {
    if seen.contains(start) {
        return 0;
    }
    seen.insert(start.to_string());

    let mut ans = 1;
    for next in h.get(start).unwrap_or(&Vec::new()) {
        ans += get_count_helper(h, next, seen);
    }
    ans
}

fn get_count(h: &HashMap<String, BagVec>, start: &str) -> i32 {
    let mut seen = HashSet::new();
    get_count_helper(h, start, &mut seen) - 1
}

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;

    let mut reverse_bag_map = HashMap::new();
    for line in buffer.trim().split('\n') {
        let (bag, contains) = parse_line(&line);

        for b in contains {
            let x = reverse_bag_map.entry(b).or_insert_with(Vec::new);
            x.push(bag.clone());
        }
    }

    println!("{}", get_count(&reverse_bag_map, "shiny gold"));

    Ok(())
}
