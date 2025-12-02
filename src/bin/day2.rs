use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct ProductID {
    start: u64,
    end: u64,
}

struct ProductIDs {
    ids: Vec<ProductID>,
}

fn is_invalid_part_one(id: u64) -> bool {
    let str_num = id.to_string();
    let half_len = (str_num.len() - str_num.len() % 2) / 2;

    let parts = str_num.split_at(half_len);

    parts.0 == parts.1
}

fn is_invalid_part_two(id: u64) -> bool {
    let str_num = id.to_string();
    if str_num.len() < 2 {
        return false;
    }
    // let max_sequence_len = (str_num.len() - str_num.len() % 2) / 2;
    let max_sequence_len = str_num.len() / 2;

    for i in 1..max_sequence_len + 1 {
        let tmp_seq = &str_num[0..i];
        let exp_repeat = str_num.len() / i;

        let test_str = tmp_seq.repeat(exp_repeat);
        if test_str == str_num {
            return true;
        }
    }

    false
}

impl ProductIDs {
    fn part_one(&self) -> u64 {
        let mut result: u64 = 0;

        for id in &self.ids {
            let mut num = id.start;

            while num <= id.end {
                if is_invalid_part_one(num) {
                    result += num;
                }
                num += 1;
            }
        }

        result
    }

    fn part_two(&self) -> u64 {
        let mut total_sum = 0;

        for id in &self.ids {
            let mut num = id.start;

            while num <= id.end {
                if is_invalid_part_two(num) {
                    total_sum += num;
                }
                num += 1;
            }
        }
        total_sum
    }
}

fn main() {
    let start = Instant::now();
    let file = File::open("data/day2.txt").expect("Could not open file");
    // let file = File::open("data/day2_test.txt").expect("Could not open file");
    let mut reader = BufReader::new(file);

    let delimiter = b',';
    let range_delimiter = '-';

    let mut product_ids = ProductIDs { ids: vec![] };

    loop {
        let mut buffer = Vec::new();
        let bytes_read = reader.read_until(delimiter, &mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        let chunk_str = String::from_utf8(buffer).expect("Invalid UTF-8");
        let trimmed_str = chunk_str.trim_end_matches(delimiter as char).trim();

        if let Some((left, right)) = trimmed_str.split_once(range_delimiter) {
            let left_num: u64 = left.parse().expect("bad num");
            let right_num: u64 = right.parse().expect("bad num");

            product_ids.ids.push(ProductID {
                start: left_num,
                end: right_num,
            });
        }
    }

    println!("Total num Part1: {}", product_ids.part_one());
    println!("Total num Part2: {}", product_ids.part_two());
    let duration = start.elapsed();
    println!("Elapsed time: {} seconds", duration.as_secs_f32());
}
