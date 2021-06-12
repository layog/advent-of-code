use std::collections::HashMap;
use std::io::Read;

type ComposeType = Vec<Vec<usize>>;

#[derive(Debug)]
enum Rule {
    Compose(ComposeType),
    Char(char),
}

fn parse_rule(s: &str) -> Rule {
    if s.starts_with('"') {
        return Rule::Char(s.chars().nth(1).unwrap());
    }

    let mut v = Vec::new();
    for rule in s.split('|').map(|x| x.trim()) {
        v.push(
            rule.split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    Rule::Compose(v)
}

fn parse_rules(s: &str) -> HashMap<usize, Rule> {
    let mut h = HashMap::new();

    for line in s.trim().split('\n') {
        let v: Vec<&str> = line.trim().split(':').map(|x| x.trim()).collect();
        assert!(v.len() == 2);

        h.insert(v[0].parse().unwrap(), parse_rule(v[1]));
    }

    h
}

// This will work because 0: 8 11. So 8 is always at the start and 11 at end.
fn special8<'a>(s: &'a str, h: &HashMap<usize, Rule>, series: &[usize]) -> (bool, &'a str) {
    let mut attempt = 1;
    while attempt < 80 {
        let mut new = vec![42;attempt];
        for &x in series[1..].iter() {
            new.push(x);
        }

        let (p, v) = check_series(s, h, &new);
        if p && v.is_empty() {
            return (p, v);
        }

        attempt += 1;
    }

    (false, s)
}

fn special11<'a>(s: &'a str, h: &HashMap<usize, Rule>, _series: &[usize]) -> (bool, &'a str) {
    let mut attempt = 1;
    while attempt < 40 {
        let mut new = vec![42;attempt];
        new.append(&mut vec![31;attempt]);

        let (p, v) = check_series(s, h, &new);
        if p && v.is_empty() {
            return (p, v);
        }

        attempt += 1;
    }

    (false, s)
}

fn check_series<'a>(s: &'a str, h: &HashMap<usize, Rule>, series: &[usize]) -> (bool, &'a str) {
    if series.is_empty() {
        return (true, s);
    }

    if series[0] == 8 {
        return special8(s, h, series);
    }

    if series[0] == 11 {
        return special11(s, h, series);
    }

    let (p, v) = burn_match(s, h, series[0]);
    if !p {
        return (false, s);
    }

    check_series(v, h, &series[1..])
}

fn burn_match<'a>(s: &'a str, h: &HashMap<usize, Rule>, idx: usize) -> (bool, &'a str) {
    let ans = match &h[&idx] {
        &Rule::Char(c) => {
            if let Some(y) = s.chars().next() {
                (c == y, &s[1..])
            } else {
                (false, s)
            }
        }
        Rule::Compose(v) => {
            let mut rval = (false, s);
            for series in v {
                let (this_match, this) = check_series(s, h, series);

                // This assumes that the number of chars in each rule will be constant
                if this_match {
                    rval = (true, this);
                    break;
                }
            }
            rval
        }
    };
    ans
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let v: Vec<&str> = buffer.trim().split("\n\n").collect();
    assert!(v.len() == 2);

    let rules = parse_rules(v[0].trim());
    let inputs: Vec<&str> = v[1].trim().split('\n').map(|x| x.trim()).collect();

    let mut ans = 0;
    for input in inputs {
        let (p, v) = burn_match(input, &rules, 0);
        if p && v.is_empty() {
            ans += 1;
        }
    }
    println!("{}", ans);
}
