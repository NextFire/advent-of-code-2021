use std::{fs, ops::RangeInclusive};

#[derive(Debug)]
struct Pair {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl Pair {
    fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(',').collect();
        let first = range_str_to_range(parts[0]);
        let second = range_str_to_range(parts[1]);
        Pair { first, second }
    }
}

fn range_str_to_range(range_str: &str) -> RangeInclusive<u32> {
    let bounds: Vec<_> = range_str.split('-').collect();
    bounds[0].parse::<u32>().unwrap()..=bounds[1].parse::<u32>().unwrap()
}

fn main() {
    let input = fs::read_to_string("inputs/04.txt").unwrap();

    let pairs: Vec<_> = input.trim().lines().map(Pair::from_line).collect();

    let resp1 = pairs
        .iter()
        .filter(|pair| {
            (pair.first.start() >= pair.second.start() && pair.first.end() <= pair.second.end())
                || (pair.second.start() >= pair.first.start()
                    && pair.second.end() <= pair.first.end())
        })
        .count();
    println!("{:?}", resp1);

    let resp2 = pairs
        .iter()
        .filter(|pair| {
            (pair.first.start() >= pair.second.start() && pair.first.end() <= pair.second.end())
                || (pair.second.start() >= pair.first.start()
                    && pair.second.end() <= pair.first.end())
                || (pair.first.start() >= pair.second.start()
                    && pair.first.start() <= pair.second.end())
                || (pair.second.start() >= pair.first.start()
                    && pair.second.start() <= pair.first.end())
        })
        .count();
    println!("{:?}", resp2);
}
