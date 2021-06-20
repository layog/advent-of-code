use std::collections::VecDeque;
use std::io::Read;

fn parse_cards(lines: &str) -> Vec<usize> {
    let lines = lines.trim().split('\n').collect::<Vec<_>>();
    lines[1..].iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let v = buffer.trim().split("\n\n").collect::<Vec<_>>();
    let mut player1 = VecDeque::from(parse_cards(v[0]));
    let mut player2 = VecDeque::from(parse_cards(v[1]));

    while !player1.is_empty() && !player2.is_empty() {
        let x1 = player1.pop_front().unwrap();
        let x2 = player2.pop_front().unwrap();

        if x2 > x1 {
            player2.push_back(x2);
            player2.push_back(x1);
        } else {
            player1.push_back(x1);
            player1.push_back(x2);
        }
    }

    let win;
    if !player1.is_empty() {
        win = player1;
    } else {
        win = player2;
    }

    let mut ans = 0;
    for (i, x) in win.into_iter().rev().enumerate() {
        ans += x * (i + 1);
    }
    println!("{}", ans);
}
