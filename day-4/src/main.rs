use std::fs;
use std::collections::HashMap;

struct Card {
    id: u32,
    winning: Vec<i32>,
    owned: Vec<i32>,
    intersection: Vec<i32>,
    copies: u32
}

impl Card {
    fn from(input: &str) -> Card {
        let a: Vec<&str> = input.split(":").collect();
        let label = a[0];
        let data = a[1];

        let id: u32 = label.split_whitespace()
            .collect::<Vec<&str>>()
            .get(1).expect("No card ID found")
            .parse().expect("Card ID not a number");

        let b: Vec<&str> = data.split("|").collect();
        let winning_nums = b[0].trim();
        let owned_nums = b[1].trim();

        let winning: Vec<i32> = winning_nums.split_whitespace().into_iter()
            .map(|x| x.parse::<i32>().expect("NaN"))
            .collect();

        let owned: Vec<i32> = owned_nums.split_whitespace().into_iter()
            .map(|x| x.parse::<i32>().expect("NaN"))
            .collect();

        let mut intersection = owned.clone();
        intersection.retain(|x| winning.contains(x));

        Card {
            id,
            winning,
            owned,
            intersection,
            copies: 1
        }
    }

    fn points(&self) -> i32 {
        let mut points = 0;
        if self.intersection.len() > 0 {
            points = 2i32.pow((self.intersection.len() as u32) - 1);
        }
        points
    }
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Failed to read file");
    let lines: Vec<&str> = contents.trim().split("\n").collect();

    let mut total_pt1 = 0;
    let mut card_map: HashMap<u32, Card> = HashMap::new();
    for line in lines {
        let cd = Card::from(line);
        total_pt1 += cd.points();
        card_map.insert(cd.id, cd);
    }

    println!("Part 1: {total_pt1}");

    let greatest_id = card_map.keys().max().unwrap();
    let mut total_pt2: u32 = 0;

    for i in 1..greatest_id+1 {
        let cd = card_map.get_mut(&i).expect("ID not found in map");
        let cd_copies = cd.copies.clone();
        for j in 0..cd.intersection.len() {
            card_map.get_mut(&(j as u32 + i + 1)).expect("ID not found in map").copies += cd_copies;
        }
        total_pt2 += cd_copies;
    }

    println!("Part 2: {total_pt2}");
}