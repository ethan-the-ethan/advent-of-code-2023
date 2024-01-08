use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Could not read file");
    let lines: Vec<&str> = input.split("\n").collect();

    let mut total_1 = 0;
    let mut total_2 = 0;
    for line in lines {
        let mut first_digits: Vec<i32> = Vec::new();
        let mut last_digits: Vec<i32> = Vec::new();
        let mut line_arr: Vec<i32> = line.split(" ")
            .map(|x| x.parse().unwrap_or(0))
            .collect();

        while line_arr.iter().fold(0, |acc, x| acc + x) != 0 {
            last_digits.push(*line_arr.last().unwrap_or(&0));
            first_digits.push(*line_arr.first().unwrap_or(&0));
            line_arr = line_arr.iter().enumerate()
                .filter_map(|(i, x)| if line_arr.get(i+1).is_some() { Some(*line_arr.get(i+1).unwrap() - x) } else { None })
                .collect();
        }

        total_1 += last_digits.iter().rev().fold(0, |acc, x| x + acc);
        total_2 += first_digits.iter().rev().fold(0, |acc, x| x - acc);
    }
    println!("Part 1: {total_1}");
    println!("Part 2: {total_2}");
}