use std::fs;
use std::collections::HashMap;

// Parse string according to the rules of part 1
// This just filters the list to include only digits and takes merges the first and last digits together
fn parse_part1(line: &str) -> i32 {
    let nums: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();
    match format!("{}{}", nums.first().unwrap_or(&'0'), nums.last().unwrap_or(&'0')).parse() {
        Ok(n) => n,
        Err(_) => 0
    }
} 

// Parse string according to the rules of part 2
fn parse_part2(line: &str) -> Option<i32> {
    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    let mut first: HashMap<usize, &str> = HashMap::new();
    let mut last: HashMap<usize, &str> = HashMap::new();

    // Search for all the words, storing index they occur mapped to the value they represent
    for word in map.keys() {
        match line.find(word) {
            Some(index) => {
                first.insert(index, map.get(word)?);
            },
            None => ()
        };

        match line.rfind(word) {
            Some(index) => {
                last.insert(index, map.get(word)?);
            },
            None => ()
        };
    }

    // As above for the numerals
    for num in map.values() {
        match line.find(num) {
            Some(index) => {
                first.insert(index, num);
            },
            None => ()
        };

        match line.rfind(num) {
            Some(index) => {
                last.insert(index, num);
            },
            None => ()
        };
    }

    // Find the very first/last index in each respective hashmap and look up the value of that index
    let firstdigit = first.get(first.keys().min()?)?;
    let lastdigit = last.get(last.keys().max()?)?;

    // Merge & Return
    match format!("{}{}", firstdigit, lastdigit).parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None
    }
}

fn main() {
    let inputfile = "src/input.txt";
    let file: String = fs::read_to_string(inputfile)
        .expect("Couldn't read file!");
    
    let lines: Vec<&str> = file.split("\n").collect();

    let mut total_1: i32 = 0;
    let mut total_2: i32 = 0;
    for l in lines {
        total_1 += parse_part1(l);
        total_2 += parse_part2(l).expect(&format!("Failed to parse line {l:?}"))
    }

    println!("Part 1: {total_1}\nPart 2: {total_2}");
}