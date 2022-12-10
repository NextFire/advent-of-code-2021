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
fn print_positions(pos: &HashSet<Position>) {
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

fn do_part1(moves: &Vec<Move>) -> HashSet<Position> {
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
        let diff_x = head_pos.x - tail_pos.x;
        let diff_y = head_pos.y - tail_pos.y;
        if diff_x.abs() == 2 || diff_y.abs() == 2 {
            tail_pos.x += diff_x.signum();
            tail_pos.y += diff_y.signum();
        }
        tail_pos_set.insert(tail_pos);
    }
    tail_pos_set
}

fn do_part2(moves: &Vec<Move>) -> HashSet<Position> {
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
            let diff_x = knobs[i - 1].x - knobs[i].x;
            let diff_y = knobs[i - 1].y - knobs[i].y;
            if diff_x.abs() == 2 || diff_y.abs() == 2 {
                knobs[i].x += diff_x.signum();
                knobs[i].y += diff_y.signum();
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

    let tail_pos_set = do_part1(&moves);
    // print_positions(&tail_pos_set);
    println!("{}", tail_pos_set.len());

    let tail_pos_set = do_part2(&moves);
    // print_positions(&tail_pos_set);
    println!("{}", tail_pos_set.len());
}
