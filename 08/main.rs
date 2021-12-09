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

        //  1111
        // 2    3
        // 2    3
        //  4444
        // 5    6
        // 5    6
        //  7777

        const VAL: Vec<&String> = Vec::new();

        for l in &input {
            let mut len_tab = [VAL; 8];

            for p in &l[0] {
                len_tab[p.len()].push(p);
            }

            let mut segments = ['_'; 7];
            let mut numbers = ["_"; 10];

            // 2 segments
            numbers[1] = len_tab[2][0];
            segments[3] = numbers[1].chars().nth(0).unwrap();
            segments[6] = numbers[1].chars().nth(1).unwrap();

            // 3 segments
            numbers[7] = len_tab[3][0];
            for c in numbers[7].chars() {
                if !segments.contains(&c) {
                    segments[1] = c;
                    break;
                }
            }

            // 4 segments
            numbers[4] = len_tab[4][0];
            for c in numbers[4].chars() {
                if !segments.contains(&c) {
                    segments[2] = c;
                    break;
                }
            }
            for c in numbers[4].chars() {
                if !segments.contains(&c) {
                    segments[4] = c;
                    break;
                }
            }

            // 5 segments

            // 6 segments

            // 7 segments
            numbers[8] = len_tab[7][0];

            println!("{:?}", segments);

            break;
        }
    }
}
