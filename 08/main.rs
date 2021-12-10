use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let input: Vec<Vec<Vec<String>>> = content
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|l| l.split(' ').map(|p| p.to_string()).collect())
                .collect()
        })
        .collect();
    println!("{:?}", input);

    println!("--- PART 1 ---");
    {
        let mut nb = 0;
        for l in &input {
            for p in &l[1] {
                if [2, 3, 4, 7].contains(&p.len()) {
                    nb += 1;
                }
            }
        }
        println!("{:?}", nb);
    }

    println!("--- PART 2 ---");
    {
        // 2 segs: 1
        // 3 segs: 7
        // 4 segs: 4
        // 5 segs: 2, 3, 5
        // 6 segs: 0, 6, 9
        // 7 segs: 8

        //  0000
        // 1    2
        // 1    2
        //  3333
        // 4    5
        // 4    5
        //  6666

        // 1 ->   2  5
        // 7 -> 0 2  5
        // 4 ->  123 5
        // sum  0123 5

        // 2 -> 0 234 6
        // 3 -> 0 23 56
        // 5 -> 01 3 56

        // 0 -> 012 456
        // 6 -> 01 3456
        // 9 -> 0123 56

        const VAL: Vec<&String> = Vec::new();

        let mut result = 0;
        for line in &input {
            let mut len_tab = [VAL; 8];

            for p in &line[0] {
                len_tab[p.len()].push(p);
            }

            let mut segments = ['_'; 7];
            let mut numbers = ["_"; 10];

            numbers[1] = len_tab[2][0];
            numbers[7] = len_tab[3][0];
            numbers[4] = len_tab[4][0];
            numbers[8] = len_tab[7][0];

            for c in numbers[7].chars() {
                if !numbers[1].contains(c) {
                    segments[0] = c;
                    break;
                }
            }

            for l in &len_tab[5] {
                let mut count_diff = 0;
                let mut last_diff = '_';
                for c in l.chars() {
                    if !numbers[4].contains(c) && c != segments[0] {
                        count_diff += 1;
                        last_diff = c;
                    }
                    if count_diff == 1 {
                        segments[6] = last_diff;
                    } else if count_diff == 2 {
                        numbers[2] = l;
                    }
                }
            }

            for c in numbers[2].chars() {
                if !numbers[4].contains(c) && !segments.contains(&c) {
                    segments[4] = c;
                    break;
                }
            }
            for l in &len_tab[5] {
                for c in l.chars() {
                    if !numbers[2].contains(c) && !numbers[7].contains(c) {
                        segments[1] = c;
                        numbers[5] = l;
                        break;
                    }
                }
            }

            for l in &len_tab[5] {
                if !numbers.contains(&l.as_str()) {
                    numbers[3] = l;
                    break;
                }
            }

            for c in numbers[1].chars() {
                if numbers[5].contains(c) {
                    segments[5] = c;
                } else {
                    segments[2] = c;
                }
            }

            for c in numbers[8].chars() {
                if !segments.contains(&c) {
                    segments[3] = c;
                    break;
                }
            }

            println!("{:?}", segments);
            println!("{:?}", len_tab[6]);
            for l in &len_tab[6] {
                if !l.contains(segments[3]) {
                    numbers[0] = l;
                } else if !l.contains(segments[2]) {
                    numbers[6] = l;
                // else if !l.contains(segments[4]) {   -- lol, it failed somewhere before
                } else if numbers[9] == "_" {
                    numbers[9] = l;
                }
            }
            println!("{:?}", numbers);

            let mut subresult = 0;
            for (i, digit) in line[1].iter().enumerate() {
                for (j, segs) in numbers.iter().enumerate() {
                    if digit.len() == segs.len() && digit.chars().all(|d| segs.contains(d)) {
                        subresult += (j as u32) * 10_u32.pow(3 - i as u32);
                    }
                }
            }
            println!("{}", subresult);
            result += subresult;
        }
        println!("{}", result);
    }
}
