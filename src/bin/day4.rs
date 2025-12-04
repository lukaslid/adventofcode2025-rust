use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),  // up
    (1, 0),   // down
    (0, -1),  // left
    (0, 1),   // right
    (-1, -1), // up-left
    (-1, 1),  // up-right
    (1, -1),  // down-left
    (1, 1),   // down-right
];
fn is_valid_position(rows: usize, cols: usize, r: i32, c: i32) -> bool {
    r >= 0 && c >= 0 && (r as usize) < rows && (c as usize) < cols
}

fn part_one(matrix: &[Vec<u32>], n: usize, m: usize) -> u32 {
    let mut result: u32 = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if matrix[i][j] == 0 {
                continue;
            }
            let mut cnt = 0;
            for d in DIRECTIONS {
                let nr = i as i32 + d.0;
                let nc = j as i32 + d.1;

                if is_valid_position(n, m, nr, nc) {
                    cnt += matrix[nr as usize][nc as usize];
                }
            }
            if cnt < 4 {
                result += 1
            }
        }
    }

    result
}

fn part_two(matrix: &mut [Vec<u32>], n: usize, m: usize) -> u32 {
    let mut result: u32 = 0;

    loop {
        let mut removed = 0;

        let snapshot = matrix.to_vec();
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if *col == 0 {
                    continue;
                }
                let mut cnt = 0;
                for d in DIRECTIONS {
                    let nr = i as i32 + d.0;
                    let nc = j as i32 + d.1;

                    if is_valid_position(n, m, nr, nc) {
                        cnt += snapshot[nr as usize][nc as usize];
                    }
                }
                if cnt < 4 {
                    removed += 1;
                    *col = 0;
                }
            }
        }
        if removed == 0 {
            break;
        }
        result += removed;
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("data/day4.txt").expect("Could not open file");
    // let file = File::open("data/day4_test.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;

    let first_line_length = lines.first().map(|line| line.len()).unwrap();

    let mut matrix = vec![vec![0; first_line_length]; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    matrix[i][j] = 1;
                }
                _ => {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    println!(
        "Total num Part1: {}",
        part_one(&matrix, lines.len(), first_line_length)
    );
    println!(
        "Total num Part1: {}",
        part_two(&mut matrix, lines.len(), first_line_length)
    );
    let duration = start.elapsed();
    println!("Elapsed time: {} seconds", duration.as_secs_f32());

    Ok(())
}
