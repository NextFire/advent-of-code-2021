use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/03.txt").unwrap();

    let doubled = input.trim().lines().map(|l| {
        let chars: Vec<_> = l.chars().collect();
        let mid = chars.len() / 2;
        for i in 0..mid {
            if chars[mid..].contains(&chars[i]) {
                return chars[i];
            }
        }
        panic!();
    });
    let sum: u32 = doubled
        .map(|c| (c as u32) - (if c.is_lowercase() { 96 } else { 38 }))
        .sum();
    println!("{:?}", sum);

    let lines: Vec<_> = input.trim().lines().collect();
    let commons = (0..(lines.len() / 3)).map(|i| {
        let group = &lines[(3 * i)..(3 * i + 3)];
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                return c;
            }
        }
        panic!();
    });
    let sum: u32 = commons
        .map(|c| (c as u32) - (if c.is_lowercase() { 96 } else { 38 }))
        .sum();
    println!("{:?}", sum);
}
