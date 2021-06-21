use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    v: usize,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(v: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            v,
            prev: None,
            next: None,
        }))
    }

    fn set_prev(&mut self, prev: Rc<RefCell<Node>>) {
        self.prev = Some(prev);
    }

    fn set_next(&mut self, next: Rc<RefCell<Node>>) {
        self.next = Some(next);
    }
}

fn walk(mut current: Rc<RefCell<Node>>, mut count: usize) -> Option<Rc<RefCell<Node>>> {
    while count != 0 {
        let x = current.borrow().next.clone()?;
        current = x;
        count -= 1;
    }
    Some(current)
}

fn walk_till_value(mut current: Rc<RefCell<Node>>, v: usize) -> Option<Rc<RefCell<Node>>> {
    while current.borrow().v != v {
        current = walk(current, 1)?;
    }
    Some(current)
}

fn join(current: Rc<RefCell<Node>>, next: Rc<RefCell<Node>>) {
    current.borrow_mut().next = Some(next.clone());
    next.borrow_mut().prev = Some(current);
}

fn get_graft_head(
    mut v: usize,
    node_map: &HashMap<usize, Rc<RefCell<Node>>>,
    graft_start: Rc<RefCell<Node>>,
    max: usize,
) -> Rc<RefCell<Node>> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut node = Some(graft_start);
    while let Some(x) = node {
        seen.insert(x.borrow().v);
        node = walk(x, 1);
    }

    v -= 1;
    if v == 0 {
        v = max;
    }
    while seen.contains(&v) {
        v -= 1;
        if v == 0 {
            v = max;
        }
    }

    node_map[&v].clone()
}

fn play_round(
    node_map: &HashMap<usize, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    max: usize,
) -> Rc<RefCell<Node>> {
    let graft_start = walk(head.clone(), 1).unwrap();
    let graft_end = walk(head.clone(), 3).unwrap();

    join(head.clone(), walk(head.clone(), 4).unwrap());

    graft_start.borrow_mut().prev = None;
    graft_end.borrow_mut().next = None;

    let new_position = get_graft_head(head.borrow().v, node_map, graft_start.clone(), max);
    let next = walk(new_position.clone(), 1).unwrap();
    join(new_position, graft_start);
    join(graft_end, next);

    walk(head, 1).unwrap()
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut v: Vec<usize> = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut node_map: HashMap<usize, Rc<RefCell<Node>>> = HashMap::new();
    let mut n = *(v.iter().max().unwrap()) + 1;
    for &x in v.iter() {
        node_map.insert(x, Node::new(x));
    }

    while n <= 1_000_000 {
        node_map.insert(n, Node::new(n));
        v.push(n);
        n += 1;
    }

    let l = v.len();
    for i in 0..l {
        let current = node_map[&v[i]].clone();
        let next = node_map[&v[(i + 1) % l]].clone();
        current.borrow_mut().set_next(next.clone());
        next.borrow_mut().set_prev(current.clone());
    }

    let n = *(v.iter().max().unwrap());
    let mut head = node_map[&v[0]].clone();
    let mut rounds = 10_000_000;
    while rounds != 0 {
        head = play_round(&node_map, head, n);
        rounds -= 1;
    }

    head = walk_till_value(head, 1).unwrap();
    let v1 = walk(head.clone(), 1).unwrap().borrow().v;
    let v2 = walk(head, 2).unwrap().borrow().v;
    println!("{}", v1 * v2);
}
