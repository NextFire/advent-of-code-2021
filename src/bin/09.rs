use std::{collections::HashSet, fs};

use advent_of_code_2022::Matrix;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
enum Move {
    Right,
    Left,
    Up,
    Down,
}

impl From<&str> for Move {
    fn from(c: &str) -> Self {
        match c {
            "R" => Move::Right,
            "L" => Move::Left,
            "U" => Move::Up,
            "D" => Move::Down,
            _ => panic!(),
        }
    }
}

#[allow(dead_code)]
fn positions_printer(pos: &HashSet<Position>) {
    let min_x = pos.iter().map(|p| p.x).min().unwrap();
    let max_x = pos.iter().map(|p| p.x).max().unwrap();
    let min_y = pos.iter().map(|p| p.y).min().unwrap();
    let max_y = pos.iter().map(|p| p.y).max().unwrap();
    let mut pos_mat: Matrix<char> =
        vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize].into();
    for p in pos {
        pos_mat.rows[(p.y - min_y) as usize][(p.x - min_x) as usize] = '#';
    }
    pos_mat.rows[(0 - min_y) as usize][(0 - min_x) as usize] = 's';
    for r in pos_mat.rows.iter().rev() {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
}

fn compute_part1(moves: &Vec<Move>) -> HashSet<Position> {
    let mut head_pos = Position { x: 0, y: 0 };
    let mut tail_pos = Position { x: 0, y: 0 };
    let mut tail_pos_set = HashSet::new();
    tail_pos_set.insert(tail_pos);
    for m in moves {
        match m {
            Move::Right => head_pos.x += 1,
            Move::Left => head_pos.x -= 1,
            Move::Up => head_pos.y += 1,
            Move::Down => head_pos.y -= 1,
        }
        if head_pos.x == tail_pos.x + 2 {
            tail_pos.x += 1;
            tail_pos.y = head_pos.y;
        }
        if head_pos.x == tail_pos.x - 2 {
            tail_pos.x -= 1;
            tail_pos.y = head_pos.y;
        }
        if head_pos.y == tail_pos.y + 2 {
            tail_pos.x = head_pos.x;
            tail_pos.y += 1;
        }
        if head_pos.y == tail_pos.y - 2 {
            tail_pos.x = head_pos.x;
            tail_pos.y -= 1;
        }
        tail_pos_set.insert(tail_pos);
    }
    tail_pos_set
}

// FIXME:
fn compute_part2(moves: &Vec<Move>) -> HashSet<Position> {
    let mut knobs = vec![Position { x: 0, y: 0 }; 10];
    let mut tail_pos_set = HashSet::new();
    tail_pos_set.insert(knobs[9]);
    for m in moves {
        match m {
            Move::Right => knobs[0].x += 1,
            Move::Left => knobs[0].x -= 1,
            Move::Up => knobs[0].y += 1,
            Move::Down => knobs[0].y -= 1,
        }
        for i in 1..=9 {
            if knobs[i - 1].x == knobs[i].x + 2 {
                knobs[i].x += 1;
                knobs[i].y = knobs[i - 1].y;
            }
            if knobs[i - 1].x == knobs[i].x - 2 {
                knobs[i].x -= 1;
                knobs[i].y = knobs[i - 1].y;
            }
            if knobs[i - 1].y == knobs[i].y + 2 {
                knobs[i].x = knobs[i - 1].x;
                knobs[i].y += 1;
            }
            if knobs[i - 1].y == knobs[i].y - 2 {
                knobs[i].x = knobs[i - 1].x;
                knobs[i].y -= 1;
            }
        }
        tail_pos_set.insert(knobs[9]);
    }
    tail_pos_set
}

fn main() {
    let input = fs::read_to_string("inputs/09.txt").unwrap();

    let moves: Vec<Move> = input
        .trim()
        .lines()
        .flat_map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            let dir: Move = parts[0].into();
            let nb: usize = parts[1].parse().unwrap();
            vec![dir; nb]
        })
        .collect();
    // println!("{:#?}", moves);

    let tail_pos_set = compute_part1(&moves);
    // positions_printer(&tail_pos_set);
    println!("{}", tail_pos_set.len());

    let tail_pos_set = compute_part2(&moves);
    // positions_printer(&tail_pos_set);
    println!("{}", tail_pos_set.len());
}
