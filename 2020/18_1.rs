use std::io::Read;

#[derive(Debug)]
enum Token {
    Add,
    Mul,
    Open,
    Close,
    Value(u64),
}

fn tokenize(mut s: &str) -> Vec<Token> {
    let mut v: Vec<Token> = Vec::new();
    s = s.trim();

    while !s.trim().is_empty() {
        let c = s.trim().chars().next().unwrap();

        if c == '(' {
            s = s[1..].trim();
            v.push(Token::Open);
        } else if c == ')' {
            s = s[1..].trim();
            v.push(Token::Close);
        } else if c == '+' {
            s = s[1..].trim();
            v.push(Token::Add);
        } else if c == '*' {
            s = s[1..].trim();
            v.push(Token::Mul);
        } else {
            let mut index = s.len();
            let tokens = [' ', ')', '+', '*'];
            for &token in tokens.iter() {
                if let Some(x) = s.find(token) {
                    if x < index {
                        index = x;
                    }
                };
            }
            v.push(Token::Value(s[..index].parse().unwrap()));
            s = s[index..].trim();
        }
    }

    v
}

fn evaluate(expression: &[Token], mut index: usize) -> (usize, u64) {
    let mut vstack = Vec::new();
    let mut operator: Option<fn(u64, u64) -> u64> = None;

    while index < expression.len() {
        let mut to_return = false;
        match expression[index] {
            Token::Add => {
                operator = Some(|x, y| x + y);
            }
            Token::Mul => {
                operator = Some(|x, y| x * y);
            }
            Token::Open => {
                let (i, value) = evaluate(expression, index + 1);
                index = i;
                vstack.push(value);
            }
            Token::Close => {
                to_return = true;
            }
            Token::Value(v) => {
                vstack.push(v);
            }
        }

        if vstack.len() == 2 {
            assert!(operator != None);
            let v1 = vstack.pop().unwrap();
            let v2 = vstack.pop().unwrap();
            if let Some(op) = operator {
                vstack.push(op(v1, v2));
                operator = None;
            }
        }

        if to_return {
            break;
        }

        index += 1;
    }

    (index, vstack[0])
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();

    let mut ans: u64 = 0;
    for expression in buffer.trim().split('\n') {
        let tokens = tokenize(expression.trim());
        ans += evaluate(&tokens, 0).1;
    }
    println!("{}", ans);
}
