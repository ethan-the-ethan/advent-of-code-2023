use std::fs;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn from(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Number {
    value: u32,
    pos: Vec<Pos>
}

#[derive(Clone, Debug)]
struct Symbol {
    value: char,
    pos: Pos,
    children: Vec<Number>
}

impl Symbol {
    fn from(x: usize, y: usize, c: char) -> Symbol {
        Symbol {
            value: c,
            pos: Pos::from(x, y),
            children: Vec::new(),
        }
    }   
}

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Failed to read file");
    let lines: Vec<&str> = contents.trim().split("\n").collect();

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut number_map: HashMap<u32, Number> = HashMap::new();

    let mut next_id = 0;

    // Parse the input file
    // x is the index within the line, y is index of line
    for (y, line) in lines.into_iter().enumerate() {
        let mut pos_vec: Vec<Pos> = Vec::new();
        let mut number_buf: String = String::new();

        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) { 
                pos_vec.push(Pos::from(x, y));
                number_buf.push(c);
            } else {
                if !number_buf.is_empty() {
                    // The number is now finished, collect data and add to number_map
                    number_map.insert(next_id, Number { value: number_buf.parse().expect("NaN"), pos: pos_vec.clone() });
                    pos_vec.clear();
                    number_buf.clear();
                    next_id += 1;
                }

                if c != '.' {
                    // Record the type and location of symbols
                    symbols.push(Symbol::from(x, y ,c));
                }
            }
        }

        // If the line ends on a number, ensure that number is still recorded
        if !number_buf.is_empty() {
            number_map.insert(next_id, Number { value: number_buf.parse().expect("NaN"), pos: pos_vec.clone() });
            pos_vec.clear();
            number_buf.clear();
            next_id += 1;
        }
    }

    // Compute the total number of valid parts
    let mut total_pt1 = 0;
    for (_, num) in number_map {
        let value = num.value;
        let loc = num.clone().pos;

        'f: for l in loc.clone() {
            for p in &mut symbols {
                if l.x.abs_diff(p.pos.x) <= 1 && l.y.abs_diff(p.pos.y) <= 1 {
                    p.children.push(num);
                    total_pt1 += value;
                    break 'f;
                }
            }
        }
    }
    
    // Compute the sum of gear ratios
    let mut total_pt2 = 0;
    for s in symbols {
        if s.value == '*' {
            if s.children.len() == 2 {
                let n1 = s.children.get(0).expect("Child vec must have index 0").value;
                let n2 = s.children.get(1).expect("Child vec must have index 1").value;
                total_pt2 += (n1 * n2);
            }
        }
    }

    println!("Part 1: {total_pt1}");
    println!("Part 2: {total_pt2}");
}