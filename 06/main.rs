use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn part_one(mut fishes: Vec<i32>) {
    for _ in 0..80 {
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                fishes.push(8);
                fishes[i] = 6;
            } else {
                fishes[i] -= 1;
            }
        }
    }
    println!("After 80 days: {} fishes", fishes.len());
}

fn part_two(fishes: Vec<i32>) {
    let mut fish_map = HashMap::new();
    for i in 0..=8 {
        fish_map.insert(i, 0);
    }
    for f in fishes {
        let v = fish_map.get_mut(&f).unwrap();
        *v += 1;
    }
    for _ in 0..256 {
        let zero_fishes = fish_map[&0];
        fish_map.insert(0, fish_map[&1]);
        fish_map.insert(1, fish_map[&2]);
        fish_map.insert(2, fish_map[&3]);
        fish_map.insert(3, fish_map[&4]);
        fish_map.insert(4, fish_map[&5]);
        fish_map.insert(5, fish_map[&6]);
        fish_map.insert(6, fish_map[&7] + zero_fishes);
        fish_map.insert(7, fish_map[&8]);
        fish_map.insert(8, zero_fishes);
    }
    println!("After 256 days: {} fishes", fish_map.values().sum::<i64>());
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let fishes: Vec<i32> = content.split(',').map(|x| x.parse().unwrap()).collect();

    part_one(fishes.clone());
    part_two(fishes.clone());
}
