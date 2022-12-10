use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/10.txt").unwrap();

    let state = input.trim().lines().fold(vec![1], |mut acc, l| {
        let last = *acc.last().unwrap();
        let parts: Vec<_> = l.split_whitespace().collect();
        match parts[..] {
            ["noop"] => acc.push(last),
            ["addx", y] => {
                acc.push(last);
                acc.push(last + y.parse::<i32>().unwrap());
            }
            _ => panic!(),
        }
        acc
    });

    let part1: i32 = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&i| i * state[(i - 1) as usize])
        .sum();
    println!("{:?}", part1);

    for (i, pos) in state.iter().enumerate() {
        let offset = (i % 40) as i32;
        if (pos - 1) <= offset && offset <= (pos + 1) {
            print!("#");
        } else {
            print!(".");
        }
        if offset == 39 {
            println!();
        }
    }
}
