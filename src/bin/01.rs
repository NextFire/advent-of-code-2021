use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    let rations = input
        .trim()
        .split("\n\n")
        .map(|elf| elf.lines().map(|ration| ration.parse::<i32>().unwrap()));
    let mut sums: Vec<_> = rations.map(|elf| elf.sum::<i32>()).collect();
    sums.sort_by(|a, b| b.cmp(a));
    println!("{}", sums[0]);
    println!("{}", sums[..3].iter().sum::<i32>())
}
