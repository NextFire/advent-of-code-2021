use std::fs;

use advent_of_code_2022::Matrix;

fn main() {
    let input = fs::read_to_string("inputs/08.txt").unwrap();

    let forest: Matrix<u32> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<_>>()
        .into();

    let (nb_row, nb_col) = forest.size();
    let mut nb_visible = 0;

    for (i, col) in forest.rows.iter().enumerate() {
        for (j, cell) in col.iter().enumerate() {
            let mut visible = vec![true; 4];
            // top
            for sub_i in 0..i {
                if forest.get(sub_i, j).unwrap() >= cell {
                    visible[0] = false;
                    break;
                }
            }
            // bottom
            for sub_i in (i + 1)..nb_row {
                if forest.get(sub_i, j).unwrap() >= cell {
                    visible[1] = false;
                    break;
                }
            }
            // left
            for sub_j in 0..j {
                if forest.get(i, sub_j).unwrap() >= cell {
                    visible[2] = false;
                    break;
                }
            }
            // right
            for sub_j in (j + 1)..nb_col {
                if forest.get(i, sub_j).unwrap() >= cell {
                    visible[3] = false;
                    break;
                }
            }
            // check
            if visible.contains(&true) {
                nb_visible += 1;
            }
        }
    }

    println!("{}", nb_visible);

    let resp2 = forest
        .rows
        .iter()
        .enumerate()
        .map(|(i, col)| {
            col.iter()
                .enumerate()
                .map(|(j, cell)| {
                    let mut score = vec![0; 4];
                    // top
                    for sub_i in (0..i).rev() {
                        score[0] += 1;
                        if forest.get(sub_i, j).unwrap() >= cell {
                            break;
                        }
                    }
                    // bottom
                    for sub_i in (i + 1)..nb_row {
                        score[1] += 1;
                        if forest.get(sub_i, j).unwrap() >= cell {
                            break;
                        }
                    }
                    // left
                    for sub_j in (0..j).rev() {
                        score[2] += 1;
                        if forest.get(i, sub_j).unwrap() >= cell {
                            break;
                        }
                    }
                    // right
                    for sub_j in (j + 1)..nb_col {
                        score[3] += 1;
                        if forest.get(i, sub_j).unwrap() >= cell {
                            break;
                        }
                    }
                    score[0] * score[1] * score[2] * score[3]
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", resp2);
}
