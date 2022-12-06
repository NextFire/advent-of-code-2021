use std::fs;

fn search_marker(stream: &Vec<char>, len: usize) -> Option<usize> {
    'outer: for i in 0..(stream.len() - len) {
        let mut char_vec = vec![false; 26];
        let end = i + len;
        for c in &stream[i..end] {
            let ind = *c as usize - 97;
            if char_vec[ind] {
                continue 'outer;
            }
            char_vec[ind] = true;
        }
        return Some(end);
    }
    return None;
}

fn main() {
    let input = fs::read_to_string("inputs/06.txt").unwrap();

    let stream: Vec<char> = input.trim().chars().collect();

    let resp1 = search_marker(&stream, 4).unwrap();
    println!("{}", resp1);

    let resp2 = search_marker(&stream, 14).unwrap();
    println!("{}", resp2);
}
