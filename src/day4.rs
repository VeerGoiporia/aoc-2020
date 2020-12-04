use regex;
use std::fs;

pub fn day_four() {
    let contents = fs::read_to_string("inputs/input_day4.txt").expect("Could not read file");
    let passports = create_passports(&contents);
    println!("-----DAY4------");
    println!("Part 1");
    part_one(passports);
    // println!("Part 2");
    // part_two(grid);
}

#[derive(Debug, Clone, Default, Copy)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}

fn part_one(passports: Vec<Passport>) {
    let bools = &passports
        .iter()
        .map(|passport| passport.is_valid())
        .collect::<Vec<bool>>();
    let mut count = 0;

    for i in bools {
        if *i {
            count += 1
        }
    }
    println!("Valid passports: {}", count);
}

fn create_passports(contents: &str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();

    // Add first passport
    passports.push(Passport::default());

    for line in contents.clone().lines() {
        // Empty line adds new empty passport to edit
        if line.is_empty() {
            passports.push(Passport::default());
            continue;
        }

        for field in line.split(' ') {
            // Allows you do modify the last passport in the list
            // this is either the first one created or one created
            // due to an empty line
            let passport = passports.last_mut().unwrap();
            // Turn blah:blah into a key and value
            let mut fields = field.split(':');
            let key = fields.next().unwrap();
            let value = fields.next().unwrap();
            match key {
                "byr" => passport.byr = Some(value),
                "iyr" => passport.iyr = Some(value),
                "eyr" => passport.eyr = Some(value),
                "hgt" => passport.hgt = Some(value),
                "hcl" => passport.hcl = Some(value),
                "ecl" => passport.ecl = Some(value),
                "pid" => passport.pid = Some(value),
                "cid" => passport.cid = Some(value),
                bad_key => panic!("Don't know what this entry is: {}", bad_key),
            }
        }
    }
    passports
}
