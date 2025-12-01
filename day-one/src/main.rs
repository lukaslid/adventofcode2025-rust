use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct Dial {
    direction: char,
    number: i32,
}

struct SafeDial {
    items: Vec<Dial>,
}

impl SafeDial {
    fn part_two(&self) -> i32 {
        let mut position: i32 = 50;
        let mut new_position: i32;
        let mut total_zero_dials = 0;

        for elem in &self.items {
            total_zero_dials += elem.number / 100;
            let remainder = elem.number % 100;
            match elem.direction {
                'L' => {
                    new_position = position - remainder;
                    total_zero_dials += (new_position <= 0 && position != 0) as i32;
                }
                'R' => {
                    new_position = position + remainder;
                    total_zero_dials += (new_position > 99) as i32;
                }
                _ => panic!("Invalid direction"),
            }
            position = new_position.rem_euclid(100);
        }

        total_zero_dials
    }

    fn part_one(&self) -> u32 {
        let mut position = 50;
        let mut total_zero_dials = 0;

        for elem in &self.items {
            match elem.direction {
                'L' => {
                    position += 100 - (elem.number % 100);
                }
                'R' => {
                    position += elem.number;
                }
                _ => panic!("Invalid direction"),
            }
            position %= 100;

            if position == 0 {
                total_zero_dials += 1;
            }
        }
        total_zero_dials
    }
}

fn main() {
    let start = Instant::now();
    let file = File::open("data/input.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut safe_dial = SafeDial { items: vec![] };

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        safe_dial.items.push(Dial {
            direction: line.chars().next().unwrap(),
            number: line[1..].parse().unwrap(),
        });
    }

    println!("Password Part One: {}", safe_dial.part_one());
    println!("Password Part Two: {}", safe_dial.part_two());

    let duration = start.elapsed();
    let nanos = duration.as_nanos();
    println!("Elapsed time: {} nanoseconds", nanos);
}
