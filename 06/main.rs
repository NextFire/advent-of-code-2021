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
    let mut fish_array = [0; 9];
    for f in fishes {
        fish_array[f as usize] += 1;
    }
    for _ in 0..256 {
        let zero_fishes = fish_array[0];
        fish_array[0] = fish_array[1];
        fish_array[1] = fish_array[2];
        fish_array[2] = fish_array[3];
        fish_array[3] = fish_array[4];
        fish_array[4] = fish_array[5];
        fish_array[5] = fish_array[6];
        fish_array[6] = fish_array[7] + zero_fishes;
        fish_array[7] = fish_array[8];
        fish_array[8] = zero_fishes;
    }
    println!("After 256 days: {} fishes", fish_array.iter().sum::<i64>());
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let fishes: Vec<i32> = content.split(',').map(|x| x.parse().unwrap()).collect();

    part_one(fishes.clone());
    part_two(fishes.clone());
}
