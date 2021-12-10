use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let matching = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    println!("--- PART 1 ---");
    {
        let error_scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
        let mut score = 0;

        for line in content.lines() {
            let mut pile = Vec::new();

            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => pile.push(c),
                    ')' | ']' | '}' | '>' => {
                        if matching[&pile.pop().unwrap()] != c {
                            score += error_scores[&c];
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }

        println!("{}", score);
    }

    println!("--- PART 2 ---");
    {
        let error_scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
        let mut scores = Vec::new();

        for line in content.lines() {
            let mut pile = Vec::new();
            let mut score: i64 = 0;
            let mut corrupted = false;

            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => pile.push(c),
                    ')' | ']' | '}' | '>' => {
                        if matching[&pile.pop().unwrap()] != c {
                            corrupted = true;
                            break;
                        }
                    }
                    _ => (),
                }
            }

            if corrupted {
                continue;
            }

            pile.reverse();
            for c in &pile {
                score *= 5;
                score += error_scores[&matching[&c]];
            }
            scores.push(score);
        }

        scores.sort();
        println!("{:?}", scores[(scores.len() - 1) / 2]);
    }
}
