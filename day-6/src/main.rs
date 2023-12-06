use std::fs;

fn calculate(time: u64, distance: u64) -> u64 {
    let mut first: u64 = 0;
    for held_time in 0..time+1 {
        if (time - held_time) * held_time > distance {
            first = held_time;
            break;
        }
    }

    let mut last: u64 = 0;
    for held_time in (0..time+1).rev() {
        if (time - held_time) * held_time > distance {
            last = held_time;
            break;
        }
    }

    (last - first) + 1
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Failed to read file");
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    
    // Part 1
    let times: Vec<u64> = lines[0].split_whitespace()
        .collect::<Vec<_>>()[1..]
        .into_iter()
        .map(|x| x.parse::<u64>().expect("NaN"))
        .collect();

    let distances: Vec<u64> = lines[1].split_whitespace()
        .collect::<Vec<_>>()[1..]
        .into_iter()
        .map(|x| x.parse::<u64>().expect("NaN"))
        .collect();

    let mut total_pt1 = 1;
    for (i, t) in times.into_iter().enumerate() {
        let d = distances[i];
        total_pt1 *= calculate(t, d);
    }

    println!("Part 1: {total_pt1}"); 

    // Part 2
    let time: u64 = lines[0].split_whitespace()
        .collect::<Vec<_>>()[1..]
        .join("")[..]
        .parse().expect("NaN");

    let distance: u64 = lines[1].split_whitespace()
        .collect::<Vec<_>>()[1..]
        .join("")[..]
        .parse().expect("NaN");

    let total_pt2 = calculate(time, distance);

    println!("Part 2: {total_pt2}");
}

/*--------------------------------------------------------------------*/

fn old_part1_solution(times: Vec<i32>, distances: Vec<i32>) -> i32 {
    let mut total_pt1: i32 = 1;
    for (i, time) in times.into_iter().enumerate() {
        let distance_goal = distances[i];
        let mut number_that_beat_goal: i32 = 0;
        for held_time in 0..time+1 {
            if (time - held_time) * held_time > distance_goal {
                number_that_beat_goal += 1;
            }
        }
        total_pt1 *= number_that_beat_goal;
    }
    total_pt1
}