use std::{fs, collections::HashMap};
use num::integer::lcm;
use regex::Regex;

fn part_1(instructions: Vec<char>, map: HashMap<String, (String, String)>) -> i32 {
    let mut iter = instructions.clone().into_iter();
    let mut target = String::from("AAA");
    let mut count = 0;
    loop {
        let instruction = match iter.next() {
            Some(n) => n,
            None => { iter = instructions.clone().into_iter(); continue; }
        };

        if target == String::from("ZZZ") {
            break;
        }

        count += 1;

        let (left, right) = map.get(&target).expect("Target does not exist");

        match instruction {
            'L' => { target = left.clone(); }
            'R' => { target = right.clone(); },
            _ => {
                println!("Invalid Instruction {instruction}");
            }
        }
    }

    count
}

fn part_2(instructions: Vec<char>, map: HashMap<String, (String, String)>) -> i64 {
    let mut iter = instructions.clone().into_iter();
    let mut targets: HashMap<usize, String> = map.keys().cloned().enumerate().collect();
    targets.retain(|_, v| v.chars().last().expect("!") == 'A');

    let mut count = 0;
    let mut first_z: HashMap<usize, i32> = HashMap::new();
    loop {
        let instruction = match iter.next() {
            Some(n) => n,
            None => { iter = instructions.clone().into_iter(); continue; }
        };

        let mut new_targets: HashMap<usize, String> = HashMap::new();
        for (i, item) in targets {
            if item.chars().collect::<Vec<char>>().last().expect("Empty Array") == &'Z' { 
                first_z.insert(i, count);
                continue;
            }

            let (left, right) = map.get(&item).expect("Target does not exist");

            match instruction {
                'L' => { new_targets.insert(i, left.clone()); }
                'R' => { new_targets.insert(i, right.clone()); },
                _ => {
                    println!("Invalid Instruction {instruction}");
                }
            }
        }

        if new_targets.len() < 1 {
            break;
        }

        targets = new_targets.clone();
        count += 1;
    }

    let mut total: i64 = 1;
    for (_, val) in first_z {
        total = lcm(total, val as i64);
    }

    total
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Could not read file");
    let mut lines = input.split("\n").collect::<Vec<&str>>();

    let instructions: Vec<char> = lines.remove(0).chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        if line.is_empty() { continue }

        let re = Regex::new(r"(?<loc>[1-9A-Z]{3}) = \((?<left>[1-9A-Z]{3}), (?<right>[1-9A-Z]{3})\)").unwrap();
        let Some(caps) = re.captures(line) else { continue };
        map.insert(String::from(&caps["loc"]), (String::from(&caps["left"]), String::from(&caps["right"])));
    }

    println!("Part 1: {}", part_1(instructions.clone(), map.clone()));
    println!("Part 2: {}", part_2(instructions, map));
}