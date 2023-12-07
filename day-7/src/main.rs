use std::{fs, collections::HashMap};

#[derive(Debug, Clone)]
enum Hand {
    FiveOfAKind(Vec<char>, i32),
    FourOfAKind(Vec<char>, i32),
    FullHouse(Vec<char>, i32),
    ThreeOfAKind(Vec<char>, i32),
    TwoPair(Vec<char>, i32),
    OnePair(Vec<char>, i32),
    HighCard(Vec<char>, i32),
}

impl Hand {
    fn power(&self) -> i32 {
        match *self {
            Hand::FiveOfAKind(_, _) => 7,
            Hand::FourOfAKind(_, _) => 6,
            Hand::FullHouse(_, _) => 5,
            Hand::ThreeOfAKind(_, _) => 4,
            Hand::TwoPair(_, _) => 3,
            Hand::OnePair(_, _) => 2,
            Hand::HighCard(_, _) => 1,
        }
    }

    fn chars(&self) -> Vec<char> {
        match self {
            Hand::FiveOfAKind(v, _) => v.clone(),
            Hand::FourOfAKind(v, _) => v.clone(),
            Hand::FullHouse(v, _) => v.clone(),
            Hand::ThreeOfAKind(v, _) => v.clone(),
            Hand::TwoPair(v, _) => v.clone(),
            Hand::OnePair(v, _) => v.clone(),
            Hand::HighCard(v, _) => v.clone(),
        }
    }

    fn bid(&self) -> i32 {
        match *self {
            Hand::FiveOfAKind(_, b) => b,
            Hand::FourOfAKind(_, b) => b,
            Hand::FullHouse(_, b) => b,
            Hand::ThreeOfAKind(_, b) => b,
            Hand::TwoPair(_, b) => b,
            Hand::OnePair(_, b) => b,
            Hand::HighCard(_, b) => b,
        }
    }

    fn wins_against(&self, other: Hand) -> bool {
        let card_vals: HashMap<char, i32> = HashMap::from([
            ('A', 14), ('K', 13), ('Q', 12),
            ('J', 11), ('T', 10), ('9', 9),
            ('8', 8), ('7', 7), ('6', 6),
            ('5', 5), ('4', 4), ('3', 3),
            ('2', 2), ('1', 1) 
        ]);

        if self.power() > other.power() { return true } 
        if self.power() < other.power() { return false }

        let binding = self.chars();
        let mut self_iter = binding.iter();

        for o in other.chars() {
            let s = self_iter.next().expect("mismatching len");
            if card_vals[&s] > card_vals[&o] { return true }
            if card_vals[&o] > card_vals[&s] { return false }
        }

        false
    }
}

fn calculate_hand(line: &str, part_2: bool) -> Hand {
    let sp: Vec<&str> = line.split(" ").collect();
    let hand = sp[0];
    let bid: i32 = sp[1].parse().expect("NaN");

    let this_hand: Vec<char> = hand.chars().clone().collect();

    let mut j = this_hand.clone();
    j.retain(|x| x == &'J');
    let j_count = j.len();

    if j_count > 0 && part_2 {
        let mut b = this_hand.clone();
        b.retain(|x| x != &'J');

        let mut count: Vec<i32> = Vec::new();
        for c in b {
            let mut a = this_hand.clone();
            a.retain(|x| x == &c);
            count.push(a.len() as i32);
        }

        count.sort();

        let return_hand = this_hand.clone().into_iter().map(|x| if x == 'J' { '1' } else { x }).collect::<Vec<_>>();

        if j_count == 5 {
            return Hand::FiveOfAKind(return_hand , bid);
        } else if j_count == 4 {
            return Hand::FiveOfAKind(return_hand, bid);
        } else if j_count == 3 {
            if count == [2, 2] {
                return Hand::FiveOfAKind(return_hand, bid);
            } else {
                return Hand::FourOfAKind(return_hand, bid);
            }
        } else if j_count == 2 {
            if count == [3, 3, 3] {
                return Hand::FiveOfAKind(return_hand, bid);
            } else if count == [1, 2, 2] {
                return Hand::FourOfAKind(return_hand, bid);
            } else if count == [1, 1, 1] {
                return Hand::ThreeOfAKind(return_hand, bid);
            }
        } else if j_count == 1 {
            if count == [4, 4, 4, 4] {
                return Hand::FiveOfAKind(return_hand , bid);
            } else if count == [1, 3, 3, 3] {
                return Hand::FourOfAKind(return_hand, bid);
            } else if count == [2, 2, 2, 2] {
                return Hand::FullHouse(return_hand, bid)
            } else if count == [1, 1, 2, 2] {
                return Hand::ThreeOfAKind(return_hand, bid)
            } else if count == [1, 1, 1, 1] {
                return Hand::OnePair(return_hand, bid)
            }
        }
    }

    let mut count: Vec<i32> = Vec::new();
    for c in hand.chars() {
        let mut a = this_hand.clone();
        a.retain(|x| x == &c);
        count.push(a.len() as i32);
    }

    count.sort();

    if count == [5; 5] {
        return Hand::FiveOfAKind(this_hand, bid);
    } else if count == [1, 4, 4, 4, 4] {
        return Hand::FourOfAKind(this_hand, bid);
    } else if count == [2, 2, 3, 3, 3] {
        return Hand::FullHouse(this_hand, bid);
    } else if count == [1, 1, 3, 3, 3] {
        return Hand::ThreeOfAKind(this_hand, bid);
    } else if count == [1, 2, 2, 2, 2] {
        return Hand::TwoPair(this_hand, bid);
    } else if count == [1, 1, 1, 2, 2] {
        return Hand::OnePair(this_hand, bid);
    } else {
        return Hand::HighCard(this_hand, bid)
    }
}

fn sort_hands(input: Vec<Hand>) -> Vec<Hand> {
    let mut new: Vec<Hand> = Vec::new();

    'a: for hand in input {
        for (i, ex) in new.clone().into_iter().enumerate() {
            if hand.wins_against(ex) {
                new.insert(i, hand.clone());
                continue 'a;
            }
        }

        new.push(hand);
    }

    new
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Failed to read file");
    let lines: Vec<&str> = contents.trim().split("\n").collect();

    let mut hands_pt1 = Vec::new();
    let mut hands_pt2 = Vec::new();
    for line in lines {
        hands_pt1.push(calculate_hand(line, false));
        hands_pt2.push(calculate_hand(line, true));
    }

    let mut sorted_pt1 = sort_hands(hands_pt1);
    sorted_pt1.reverse();
    
    let mut total_pt1 = 0;
    for (i, hand) in sorted_pt1.into_iter().enumerate() {
        total_pt1 += (i as i32 + 1) * hand.bid();
    }

    println!("Part 1: {total_pt1}");

    let mut sorted_pt2 = sort_hands(hands_pt2);
    sorted_pt2.reverse();
    
    let mut total_pt2 = 0;
    for (i, hand) in sorted_pt2.into_iter().enumerate() {
        total_pt2 += (i as i32 + 1) * hand.bid();
    }

    println!("Part 2: {total_pt2}");
}
