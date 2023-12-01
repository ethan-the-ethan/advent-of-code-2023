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

    let mut v: Vec<&&str> = map.keys().collect(); 
    v.extend(map.values());

    // Search for all the words and numerals, storing index they occur mapped to the value they represent
    for word in v {
        match line.find(word) {
            Some(index) => first.insert(index, map.get(word).unwrap_or(word)),
            None => continue
        };

        match line.rfind(word) {
            Some(index) => last.insert(index, map.get(word).unwrap_or(word)),
            None => continue
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

// Tests to ensure what I'm changing doesn't make the algorithm wrong
#[cfg(test)]
mod tests {
    use crate::{parse_part1, parse_part2};
    #[test]
    fn part1() {
        assert_eq!(parse_part1("treb7uchet"), 77);
        assert_eq!(parse_part1("pqr3stu8vwx"), 38);
    }
    #[test]
    fn part2() {
        assert_eq!(parse_part2("7pqrstsixteen"), Some(76));
        assert_eq!(parse_part2("1sevenmpbvtgfivefive"), Some(15));
        assert_eq!(parse_part2("hjafive2klasnine"), Some(59));
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