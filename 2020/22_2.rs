use std::collections::{HashSet, VecDeque};
use std::io::Read;

fn parse_cards(lines: &str) -> Vec<usize> {
    let lines = lines.trim().split('\n').collect::<Vec<_>>();
    lines[1..].iter().map(|x| x.parse().unwrap()).collect()
}

fn extract(v: &VecDeque<usize>, s: usize, e: usize) -> VecDeque<usize> {
    VecDeque::from(v.iter().skip(s).take(e).cloned().collect::<Vec<_>>())
}

fn game(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> (usize, VecDeque<usize>) {
    let mut seen1: HashSet<VecDeque<usize>> = HashSet::new();
    let mut seen2: HashSet<VecDeque<usize>> = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        if seen1.contains(&player1) || seen2.contains(&player2) {
            return (0, player1);
        }

        seen1.insert(player1.clone());
        seen2.insert(player2.clone());

        let x1 = player1.pop_front().unwrap();
        let x2 = player2.pop_front().unwrap();

        let win;
        if player1.len() >= x1 && player2.len() >= x2 {
            win = game(extract(&player1, 0, x1), extract(&player2, 0, x2)).0;
        } else {
            win = (x2 > x1) as usize;
        }

        if win == 1 {
            player2.push_back(x2);
            player2.push_back(x1);
        } else {
            player1.push_back(x1);
            player1.push_back(x2);
        }
    }

    if !player1.is_empty() {
        (0, player1)
    } else {
        (1, player2)
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let v = buffer.trim().split("\n\n").collect::<Vec<_>>();
    let player1 = VecDeque::from(parse_cards(v[0]));
    let player2 = VecDeque::from(parse_cards(v[1]));

    let win = game(player1, player2).1;

    let mut ans = 0;
    for (i, x) in win.into_iter().rev().enumerate() {
        ans += x * (i + 1);
    }
    println!("{}", ans);
}
