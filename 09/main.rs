use std::fs::File;
use std::io::Read;

fn get_low_pts(input: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut lpts = Vec::new();
    for (i, line) in input.iter().enumerate() {
        for (j, &val) in line.iter().enumerate() {
            if i != 0 && input[i - 1][j] < val {
                continue;
            }
            if j != 0 && input[i][j - 1] < val {
                continue;
            }
            if j != line.len() - 1 && input[i][j + 1] < val {
                continue;
            }
            if i != input.len() - 1 && input[i + 1][j] < val {
                continue;
            }
            lpts.push((i, j));
        }
    }
    lpts
}

fn get_high_around(acc: &mut Vec<(usize, usize)>, input: &Vec<Vec<u32>>, &(i, j): &(usize, usize)) {
    if i != 0 && input[i - 1][j] < 9 {
        if !acc.contains(&(i - 1, j)) {
            acc.push((i - 1, j));
            get_high_around(acc, input, &(i - 1, j));
        }
    }
    if j != 0 && input[i][j - 1] < 9 {
        if !acc.contains(&(i, j - 1)) {
            acc.push((i, j - 1));
            get_high_around(acc, input, &(i, j - 1));
        }
    }
    if j != input[i].len() - 1 && input[i][j + 1] < 9 {
        if !acc.contains(&(i, j + 1)) {
            acc.push((i, j + 1));
            get_high_around(acc, input, &(i, j + 1));
        }
    }
    if i != input.len() - 1 && input[i + 1][j] < 9 {
        if !acc.contains(&(i + 1, j)) {
            acc.push((i + 1, j));
            get_high_around(acc, input, &(i + 1, j));
        }
    }
}

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
        let low_pts = get_low_pts(&input);
        let risk: u32 = low_pts.iter().map(|&(i, j)| input[i][j] + 1).sum();
        println!("risk: {}", risk);
    }

    println!("--- PART 2 ---");
    {
        let low_pts = get_low_pts(&input);
        let bassins: Vec<Vec<(usize, usize)>> = low_pts
            .iter()
            .map(|pt| {
                let mut bassin = vec![*pt];
                get_high_around(&mut bassin, &input, pt);
                bassin
            })
            .collect();

        let mut bassins_len: Vec<usize> = bassins.iter().map(|b| b.len()).collect();
        bassins_len.sort();
        bassins_len.reverse();

        let mut result = 1;
        for len in &bassins_len[0..3] {
            result *= len;
        }
        println!("bassins: {}", result);
    }
}
