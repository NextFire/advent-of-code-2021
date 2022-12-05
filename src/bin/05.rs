use std::fs;

#[derive(Debug)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

impl Move {
    fn from_line(l: &str) -> Self {
        let parts: Vec<u32> = l.split(',').map(|c| c.parse().unwrap()).collect();
        Move {
            count: parts[0],
            from: parts[1],
            to: parts[2],
        }
    }
}

fn main() {
    let input_a = fs::read_to_string("inputs/05a.txt").unwrap();
    let input_b = fs::read_to_string("inputs/05b.txt").unwrap();

    let stacks: Vec<Vec<char>> = input_a
        .trim()
        .lines()
        .map(|l| l.split(',').map(|c| c.parse().unwrap()).collect())
        .collect();
    let moves: Vec<Move> = input_b.trim().lines().map(|l| Move::from_line(l)).collect();

    let mut stacks1 = stacks.clone();
    for mov in &moves {
        for _ in 0..mov.count {
            let poped = stacks1[mov.from as usize - 1].pop().unwrap();
            stacks1[mov.to as usize - 1].push(poped);
        }
    }
    println!(
        "{:?}",
        stacks1.iter().map(|v| v[v.len() - 1]).collect::<Vec<_>>()
    );

    let mut stacks2 = stacks.clone();
    for mov in &moves {
        let mut buf = Vec::new(); // I'm lazy
        for _ in 0..mov.count {
            let poped = stacks2[mov.from as usize - 1].pop().unwrap();
            buf.push(poped)
        }
        for _ in 0..mov.count {
            let poped = buf.pop().unwrap();
            stacks2[mov.to as usize - 1].push(poped);
        }
    }
    println!(
        "{:?}",
        stacks2.iter().map(|v| v[v.len() - 1]).collect::<Vec<_>>()
    );
}
