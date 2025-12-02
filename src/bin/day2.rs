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

fn is_valid_num(num: u64) -> bool {
    let str_num = num.to_string();
    let half_len = (str_num.len() - str_num.len() % 2) / 2;

    for i in 0..half_len {
        if !(str_num.len()).is_multiple_of(i + 1) {
            continue;
        }
        let hmap = char_freq(&str_num, i + 1);

        for (_c, v) in hmap {
            match i + 1 {
                1 => {
                    if v == str_num.len() {
                        return false;
                    }
                }
                _ => {
                    if v == str_num.len() / (i + 1) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn char_freq(s: &str, char_range: usize) -> HashMap<&str, usize> {
    let mut counter = HashMap::new();
    let length = s.len();

    for i in (0..length).step_by(char_range) {
        if i + char_range > length {
            break;
        }
        *counter.entry(&s[i..i + char_range]).or_insert(0) += 1;
    }
    counter
}

impl ProductIDs {
    fn calculate_invalid_ids_sum(&self) -> u64 {
        let mut result: u64 = 0;

        for id in &self.ids {
            let mut num = id.start;

            while num <= id.end {
                if !is_valid_num(num) {
                    result += num;
                }
                num += 1;
            }
        }

        result
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
    println!("Total num: {}", product_ids.calculate_invalid_ids_sum());
    let duration = start.elapsed();
    println!("Elapsed time: {} seconds", duration.as_secs_f32());
}
