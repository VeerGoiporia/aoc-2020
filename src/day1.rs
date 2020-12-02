use std::fs;

pub fn day_one() {
    println!("-----DAY1------");
    let numbers: Vec<i32> = convert_file_to_nums("input_day1.txt");
    let target_sum = 2020;

    println!("Part 1");
    part_one(numbers.clone(), target_sum);
    println!("Part 2");
    part_two(numbers, target_sum);
}

fn part_one(numbers: Vec<i32>, target_sum: i32) {
    let found = find_two_values(numbers, target_sum);

    if found == false {
        println!("Could not find two values that summed to {}", target_sum);
    }
}

fn part_two(numbers: Vec<i32>, target_sum: i32) {
    let found = find_three_values(numbers, target_sum);

    if found == false {
        println!("Could not find three values that summed to {}", target_sum);
    }
}

fn find_two_values(numbers: Vec<i32>, target: i32) -> bool {
    for (index, first) in numbers.iter().enumerate() {
        for second in &numbers[index + 1..] {
            if first + second == target {
                println!(
                    "Values found! First value is {} and second value is {}",
                    first, second
                );
                let mult = first * second;
                println!("Multiple of values is {}", mult);
                return true;
            }
        }
    }
    return false;
}

fn find_three_values(numbers: Vec<i32>, target: i32) -> bool {
    for (index, first) in numbers.iter().enumerate() {
        for second in &numbers[index + 1..] {
            for third in &numbers[index + 2..] {
                if first + second + third == target {
                    println!(
                        "Values found! First value is {}, second value is {} and third value is {}",
                        first, second, third
                    );
                    let mult = first * second * third;
                    println!("Multiple of values is {}", mult);
                    return true;
                }
            }
        }
    }
    return false;
}

fn convert_file_to_nums(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut numbers: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let number: i32 = line.parse().unwrap();
        numbers.push(number);
    }
    numbers
}
