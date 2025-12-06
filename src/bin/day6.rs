use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::time::Instant;

use std::error::Error;

#[derive(Debug)]
struct FreshIngredetientsDB {
    ranges: Vec<Range<u64>>,
}

fn part_one(file: &File) -> i64 {
    let reader = BufReader::new(file);
    let mut cols: Vec<Vec<i64>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let mut lines = reader.lines();

    for line_result in lines.by_ref() {
        let line = line_result.expect("parse err");
        if line.trim().is_empty() {
            continue;
        }

        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if !nums.is_empty() {
            while cols.len() < nums.len() {
                cols.push(Vec::new());
            }

            for (i, &num) in nums.iter().enumerate() {
                cols[i].push(num);
            }
        } else {
            ops = line.chars().filter(|c| *c == '*' || *c == '+').collect();
            // If the line is empty or you want to break here before ops reading
            break;
        }
    }
    let mut count = 0;
    println!("cokls {:?}, ops {:?}", cols, ops);

    for (i, col) in cols.iter().enumerate() {
        let op = ops.get(i).copied().unwrap();
        count += match op {
            '*' => col.iter().fold(1i64, |acc, &x| acc * x),
            '+' => col.iter().map(|&x| x).sum::<i64>(),
            _ => panic!("Unsupported operation: {}", op),
        };

        println!("CNT {}", count);
    }

    count
}

fn part_two(file: &File) -> i64 {
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|r| r.unwrap()).collect();
    print!("LINE {}", lines.len());
    let (num_lines, op_line) = lines.split_at(lines.len() - 1);
    let op_line = &op_line[0];

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut count: i64 = 0;
    let mut current_cols: Vec<i64> = Vec::new();
    let mut current_op: char = '+';

    for col in (0..max_width).rev() {
        let chars: Vec<char> = num_lines
            .iter()
            .map(|l| l.chars().nth(col).unwrap_or(' '))
            .collect();

        let all_spaces = chars.iter().all(|&c| !c.is_ascii_digit());

        if all_spaces {
            if !current_cols.is_empty() {
                count += match current_op {
                    '*' => current_cols.iter().fold(1i64, |acc, &x| acc * x),
                    '+' => current_cols.iter().sum::<i64>(),
                    _ => panic!("Unsupported operation: {}", current_op),
                };
                current_cols.clear();
            }
        } else {
            let num: i64 = chars
                .iter()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            current_cols.push(num);
        }

        let Some(op) = op_line.chars().nth(col) else {
            continue;
        };
        if op == '*' || op == '+' {
            current_op = op;
        }
    }

    // Finalize last problem
    if !current_cols.is_empty() {
        count += match current_op {
            '*' => current_cols.iter().fold(1i64, |acc, &x| acc * x),
            '+' => current_cols.iter().sum::<i64>(),
            _ => panic!("Unsupported operation: {}", current_op),
        };
    }

    count
}
fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("data/day6.txt").expect("Could not open file");
    let file = File::open("data/day6_test.txt").expect("Could not open file");

    println!("Part 1: {}", part_one(&file));
    println!("Part 2: {}", part_two(&file));

    let duration = start.elapsed();
    println!("Elapsed time: {} seconds", duration.as_secs_f32());

    Ok(())
}
