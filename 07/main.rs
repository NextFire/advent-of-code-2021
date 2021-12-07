use std::fs::File;
use std::io::Read;

fn part_one(mut crabs: Vec<i32>) {
    crabs.sort();
    let middle = crabs.len() / 2;
    let median = match crabs.len() % 2 {
        0 => (crabs[middle] + crabs[middle - 1]) / 2,
        _ => crabs[middle],
    };
    println!("median: {}", median);
    let fuel: i32 = crabs.iter().map(|c| (c - median).abs()).sum();
    println!("fuel: {}", fuel);
}

fn part_two(crabs: Vec<i32>) {
    let max = crabs.iter().max().unwrap();
    let mut crab_vec = vec![0; *max as usize + 1];
    crabs.iter().for_each(|c| crab_vec[*c as usize] += 1);
    let mut min_pos = 0;
    let mut min_fuel = i32::MAX;
    for pos in 0..crab_vec.len() {
        let curr_fuel: i32 = crab_vec
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let dist = (i as i32 - pos as i32).abs();
                n * (dist * (dist + 1)) / 2
            })
            .sum();
        if curr_fuel < min_fuel {
            min_pos = pos;
            min_fuel = curr_fuel;
        }
    }
    println!("min pos {} with {} fuel", min_pos, min_fuel);
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let crabs: Vec<i32> = content.split(',').map(|x| x.parse().unwrap()).collect();

    println!("PART 1");
    part_one(crabs.clone());
    println!("PART 2");
    part_two(crabs.clone());
}
