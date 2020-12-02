use recap::Recap;
use serde::Deserialize;
use std::fs;

pub fn day_two() {
    println!("-----DAY2------");
    println!("Part 1");
    part_one();
    println!("Part 2");
    part_two();
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<first_num>.*)-(?P<second_num>.*) (?P<char>[a-z]): (?P<pwd>.*)")]
struct Entry {
    first_num: i32,
    second_num: i32,
    char: char,
    pwd: String,
}

impl Entry {
    fn validate_rule1(self) -> bool {
        let count = self.pwd.matches(self.char).count() as i32;
        if count >= self.first_num && count <= self.second_num {
            return true;
        } else {
            return false;
        }
    }

    fn validate_rule2(self) -> bool {
        (self.pwd.chars().nth((self.first_num - 1) as usize).unwrap() == self.char)
            ^ (self
                .pwd
                .chars()
                .nth((self.second_num - 1) as usize)
                .unwrap()
                == self.char)
    }
}

fn part_one() {
    let entries: Vec<Entry> = convert_line_to_entry("input_day2.txt");
    let mut count: i32 = 0;
    for entry in entries {
        if entry.validate_rule1() {
            count += 1;
        }
    }

    println!("Valid password count (Rule-set 1): {}", count);
}

fn part_two() {
    let entries: Vec<Entry> = convert_line_to_entry("input_day2.txt");
    let mut count: i32 = 0;
    for entry in entries {
        if entry.validate_rule2() {
            count += 1;
        }
    }
    println!("Valid password count (Rule-set 2): {}", count);
}

fn convert_line_to_entry(filename: &str) -> Vec<Entry> {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut entries: Vec<Entry> = Vec::new();
    for line in contents.lines() {
        let entry: Entry = line.parse().unwrap();
        entries.push(entry);
    }
    entries
}
