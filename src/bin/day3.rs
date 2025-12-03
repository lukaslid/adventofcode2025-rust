use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part_one(vecs: &Vec<Vec<u8>>) -> u32 {
    let mut result: u32 = 0;

    for bank in vecs {
        let mut first_id = 0;

        // could be done in two pointers..

        for (i, num) in bank[0..bank.len() - 1].iter().enumerate() {
            if num > bank.get(first_id).unwrap() {
                first_id = i;
            }
        }

        let second = bank[first_id + 1..].iter().max().cloned().unwrap();

        result += bank.get(first_id).cloned().unwrap() as u32 * 10 + second as u32;
    }

    result
}

fn part_two(vecs: &Vec<Vec<u8>>) -> u64 {
    let mut result: u64 = 0;

    for bank in vecs {
        let mut prev_max_id = 0;
        let mut largest_joltage: u64 = 0;

        for i in (0..12).rev() {
            let mut local_max: u8 = 0;
            let mut local_max_id = prev_max_id;

            for (j, &num) in bank[prev_max_id..bank.len() - i].iter().enumerate() {
                if num > local_max {
                    local_max = num;
                    local_max_id = j + prev_max_id;
                }
            }

            largest_joltage += (local_max) as u64 * 10_u64.pow(i as u32);
            prev_max_id = local_max_id + 1;
        }
        result += largest_joltage;
    }

    result
}

fn main() {
    let start = Instant::now();
    // let file = File::open("data/day3.txt").expect("Could not open file");
    let file = File::open("data/day3_test.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let mut digit_vecs: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let mut digits: Vec<u8> = vec![];
        // line = line.expect("failed to read");

        for c in line.unwrap().as_str().chars() {
            let digit = c.to_digit(10).unwrap();
            digits.push(digit as u8)
        }

        digit_vecs.push(digits);
    }

    println!("Total num Part1: {}", part_one(&digit_vecs));
    println!("Total num Part1: {}", part_two(&digit_vecs));
    let duration = start.elapsed();
    println!("Elapsed time: {} seconds", duration.as_secs_f32());
}
