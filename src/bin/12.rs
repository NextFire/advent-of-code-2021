use std::{collections::HashSet, fs};

use advent_of_code_2022::Matrix;

fn do_dijkstra(heightmap: &Matrix<char>, start_i: usize, start_j: usize) -> Matrix<Option<u32>> {
    let size = heightmap.size();

    let mut dijkstra: Matrix<Option<u32>> = vec![vec![None; size.1]; size.0].into();
    dijkstra.set(start_i, start_j, Some(0));

    let offsets = [(-1, 0), (0, 1), (0, -1), (1, 0)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let ((min_i, min_j), &min) = match dijkstra
            .rows
            .iter()
            .enumerate()
            .flat_map(|(i, r)| {
                r.iter()
                    .enumerate()
                    .map(|(j, x)| match x {
                        Some(v) if !visited.contains(&(i, j)) => Some(((i, j), v)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .filter_map(|x| x)
            .min_by(|(_, x), (_, y)| x.cmp(y))
        {
            Some(x) => x,
            None => break,
        };

        let curr = heightmap.get(min_i, min_j).unwrap();

        for (oi, oj) in offsets {
            let ni = (min_i as i32) + oi;
            let nj = (min_j as i32) + oj;
            if ni >= 0 && nj >= 0 {
                let ni = ni as usize;
                let nj = nj as usize;
                if let Some(neighbour) = heightmap.get(ni, nj) {
                    if *curr == 'S' && *neighbour == 'a'
                        || *curr == 'z' && *neighbour == 'E'
                        || (neighbour.is_lowercase() && (*neighbour as u32) <= (*curr as u32))
                        || (neighbour.is_lowercase() && (*neighbour as u32) == (*curr as u32) + 1)
                    {
                        dijkstra.set(
                            ni,
                            nj,
                            match dijkstra.get(ni, nj).unwrap() {
                                Some(v) => Some((*v).min(min + 1)),
                                None => Some(min + 1),
                            },
                        );
                    }
                }
            }
        }

        visited.insert((min_i, min_j));
    }

    dijkstra
}

fn main() {
    let input = fs::read_to_string("inputs/12.txt").unwrap();

    let heightmap: Matrix<char> = input
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>()
        .into();
    let end_pos = heightmap.find('E').unwrap();

    // Part 1
    let start_pos = heightmap.find('S').unwrap();
    let disjkstra = do_dijkstra(&heightmap, start_pos.0, start_pos.1);
    println!("{:?}", disjkstra.get(end_pos.0, end_pos.1).unwrap());
}
