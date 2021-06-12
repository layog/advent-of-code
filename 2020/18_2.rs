use std::io::Read;

#[derive(Debug, PartialEq)]
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

fn perform(t: Token, v1: u64, v2: u64) -> u64 {
    match t {
        Token::Add => v1 + v2,
        Token::Mul => v1 * v2,
        _ => {
            panic!("No other operator");
        }
    }
}

fn evaluate(expression: &[Token], mut index: usize) -> (usize, u64) {
    let mut vstack = Vec::new();
    let mut ostack: Vec<Token> = Vec::new();

    while index < expression.len() {
        let mut to_return = false;
        match expression[index] {
            Token::Add => {
                ostack.push(Token::Add);
            }
            Token::Mul => {
                ostack.push(Token::Mul);
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

        if expression[index] != Token::Add
            && !ostack.is_empty()
            && ostack[ostack.len() - 1] == Token::Add
            && vstack.len() > 1
        {
            let v1 = vstack.pop().unwrap();
            let v2 = vstack.pop().unwrap();
            vstack.push(perform(ostack.pop().unwrap(), v1, v2));
        }

        if to_return {
            break;
        }

        index += 1;
    }

    while !ostack.is_empty() {
        let v1 = vstack.pop().unwrap();
        let v2 = vstack.pop().unwrap();
        vstack.push(perform(ostack.pop().unwrap(), v1, v2));
    }

    assert!(vstack.len() == 1);
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
