use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let input: Vec<Vec<u32>> = content
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("{:?}", input);

    println!("--- PART 1 ---");
    {
        let mut input: Vec<Vec<u32>> = input.clone();
        let mut tot = 0;

        for _ in 0..100 {
            // inc
            for row in input.iter_mut() {
                for value in row.iter_mut() {
                    *value += 1;
                }
            }

            loop {
                let mut bloom = 0;
                for i in 0..input.len() {
                    for j in 0..input[i].len() {
                        if input[i][j] >= 10 {
                            input[i][j] = 0;
                            bloom += 1;
                            if i != 0 && j != 0 && input[i - 1][j - 1] != 0 {
                                input[i - 1][j - 1] += 1;
                            }
                            if i != 0 && input[i - 1][j] != 0 {
                                input[i - 1][j] += 1;
                            }
                            if i != 0 && j != input[i].len() - 1 && input[i - 1][j + 1] != 0 {
                                input[i - 1][j + 1] += 1;
                            }
                            if j != 0 && input[i][j - 1] != 0 {
                                input[i][j - 1] += 1;
                            }
                            if j != input[i].len() - 1 && input[i][j + 1] != 0 {
                                input[i][j + 1] += 1;
                            }
                            if i != input.len() - 1 && j != 0 && input[i + 1][j - 1] != 0 {
                                input[i + 1][j - 1] += 1;
                            }
                            if i != input.len() - 1 && input[i + 1][j] != 0 {
                                input[i + 1][j] += 1;
                            }
                            if i != input.len() - 1
                                && j != input.len() - 1
                                && input[i + 1][j + 1] != 0
                            {
                                input[i + 1][j + 1] += 1;
                            }
                        }
                    }
                }
                tot += bloom;
                if bloom == 0 {
                    break;
                }
            }
        }

        println!("{:?}", tot);
    }

    println!("--- PART 2 ---");
    {
        let mut input: Vec<Vec<u32>> = input.clone();

        let mut step = 0;
        loop {
            step += 1;

            // inc
            for row in input.iter_mut() {
                for value in row.iter_mut() {
                    *value += 1;
                }
            }

            loop {
                let mut bloom = 0;
                for i in 0..input.len() {
                    for j in 0..input[i].len() {
                        if input[i][j] >= 10 {
                            input[i][j] = 0;
                            bloom += 1;
                            if i != 0 && j != 0 && input[i - 1][j - 1] != 0 {
                                input[i - 1][j - 1] += 1;
                            }
                            if i != 0 && input[i - 1][j] != 0 {
                                input[i - 1][j] += 1;
                            }
                            if i != 0 && j != input[i].len() - 1 && input[i - 1][j + 1] != 0 {
                                input[i - 1][j + 1] += 1;
                            }
                            if j != 0 && input[i][j - 1] != 0 {
                                input[i][j - 1] += 1;
                            }
                            if j != input[i].len() - 1 && input[i][j + 1] != 0 {
                                input[i][j + 1] += 1;
                            }
                            if i != input.len() - 1 && j != 0 && input[i + 1][j - 1] != 0 {
                                input[i + 1][j - 1] += 1;
                            }
                            if i != input.len() - 1 && input[i + 1][j] != 0 {
                                input[i + 1][j] += 1;
                            }
                            if i != input.len() - 1
                                && j != input.len() - 1
                                && input[i + 1][j + 1] != 0
                            {
                                input[i + 1][j + 1] += 1;
                            }
                        }
                    }
                }
                if bloom == 0 {
                    break;
                }
            }

            let mut all_bloom = true;
            for row in &input {
                for &v in row {
                    if v != 0 {
                        all_bloom = false;
                        break;
                    }
                }
                if !all_bloom {
                    break;
                }
            }

            if all_bloom {
                println!("{:?}", step);
                break;
            }
        }
    }
}
