use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::time::Instant;

use std::error::Error;

#[derive(Debug)]
struct FreshIngredetientsDB {
    ranges: Vec<Range<u64>>,
}

fn part_one(fresh_db: &FreshIngredetientsDB, available_vec: Vec<u64>) -> i32 {
    let mut count = 0;

    for i in available_vec {
        for rng in &fresh_db.ranges {
            if rng.contains(&i) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part_two(fresh_db: &mut FreshIngredetientsDB) -> u64 {
    let mut optimized_db: FreshIngredetientsDB = FreshIngredetientsDB { ranges: vec![] };

    fresh_db.ranges.sort_by_key(|r| r.start);

    for rng in fresh_db.ranges.iter() {
        let mut insert = true;

        for optimal_rng in optimized_db.ranges.iter_mut() {
            if optimal_rng.start <= rng.start && optimal_rng.end >= rng.end {
                *optimal_rng = optimal_rng.start..optimal_rng.end;
                insert = false;
            } else if optimal_rng.start >= rng.start && optimal_rng.end <= rng.end {
                *optimal_rng = rng.start..rng.end;
                insert = false;
            } else if optimal_rng.start >= rng.start && optimal_rng.end >= rng.end {
                insert = false;
                *optimal_rng = rng.start..optimal_rng.end;
            } else if optimal_rng.start <= rng.start
                && optimal_rng.end <= rng.end
                && optimal_rng.end >= rng.start
            {
                *optimal_rng = optimal_rng.start..rng.end;
                insert = false;
            }
        }
        if insert {
            optimized_db.ranges.push(rng.start..rng.end);
        }
    }

    optimized_db
        .ranges
        .iter()
        .map(|r| r.end.saturating_sub(r.start) + 1)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("data/day5.txt").expect("Could not open file");
    let file = File::open("data/day5_test.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut fresh_db = FreshIngredetientsDB { ranges: vec![] };

    let mut available_vec: Vec<u64> = vec![];

    let mut read_opt: u8 = 0; // 0 - fresh ranges. 1 - available 

    for line in reader.lines() {
        let line = line.expect("parse err");

        if line.is_empty() {
            read_opt = 1;
        } else if read_opt == 1 {
            available_vec.push(line.parse().unwrap());
        } else {
            let parts = line.split_once('-').unwrap();
            let start: u64 = parts.0.parse().unwrap();
            let end: u64 = parts.1.parse().unwrap();

            fresh_db.ranges.push(start..end);
        }
    }

    println!("Part 1: {}", part_one(&fresh_db, available_vec));
    println!("Part 2: {}", part_two(&mut fresh_db));

    let duration = start.elapsed();
    println!("Elapsed time: {} seconds", duration.as_secs_f32());

    Ok(())
}
