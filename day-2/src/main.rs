use std::fs;

struct Game {
    id: u32,
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>
}

impl Game {
    fn new() -> Game {
        Game {
            id: 0,
            reds: vec![],
            greens: vec![],
            blues: vec![]
        }
    }

    fn from(input: &str) -> Option<Game> {
        let mut gm = Game::new();

        let a: Vec<&str> = input.split(':').collect();
        let game_title = a.get(0)?;
        let game_contents = a.get(1)?;

        let b: Vec<&str> = game_title.split(" ").collect();
        gm.id = b.get(1)?.parse().expect("Provided ID not an integer");

        let subgames: Vec<&str> = game_contents
            .split(";")
            .into_iter()
            .map(|x| x.trim())
            .collect();

        for game in subgames {
            let items = game.split(", ").into_iter();
            let mut colours = (0, 0, 0);
            for item in items {
                let c: Vec<&str> = item.split(" ").collect();
                let count: u32 = c.get(0)?.parse().expect("Provided count not an integer");
                let colour = c.get(1)?;
                match colour {
                    &"red" => colours.0 = count,
                    &"green" => colours.1 = count,
                    &"blue" => colours.2 = count,
                    _ => ()
                }
            }

            gm.reds.push(colours.0);
            gm.greens.push(colours.1);
            gm.blues.push(colours.2);
        }

        Some(gm)
    }

    fn is_valid_for(&self, reds: u32, greens: u32, blues: u32) -> bool {
        let max_red: u32 = *self.reds.iter().max().expect("Empty Array");
        let max_green: u32 = *self.greens.iter().max().expect("Empty Array");
        let max_blue: u32 = *self.blues.iter().max().expect("Empty Array");
        if (max_red > reds) || (max_green > greens) || (max_blue > blues) {
            return false;
        }
        true
    }

    fn power_of_min(&self) -> u32 {
        self.reds.iter().max().unwrap() * self.greens.iter().max().unwrap() * self.blues.iter().max().unwrap()
    }
}

fn main() {
    let filepath = "src/input.txt";
    let input = fs::read_to_string(filepath).expect("Couldn't read file");

    let lines: Vec<&str> = input.trim().split("\n").collect();

    let ids: Vec<u32> = lines.clone().into_iter()
        .map(|x| Game::from(x).expect("Failed to interpret line"))
        .filter(|x| x.is_valid_for(12, 13, 14))
        .map(|x| x.id)
        .collect();

    println!("Part 1: {}", ids.iter().sum::<u32>());

    let power: u32 = lines.clone().into_iter()
        .map(|x| Game::from(x).expect("Failed to interpret line").power_of_min())
        .sum();

    println!("Part 2: {power}");
}
