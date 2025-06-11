use std::fs;

use two::*;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let lines = file.split("\n");

    let mut presents = vec![];

    for line in lines {
        if line == "" {
            break;
        }
        presents.push(Present::from_str(line));
    }

    let paper_needed = presents.iter().map(|x| x.paper_needed()).sum::<u32>();
    let ribbon_needed = presents.iter().map(|x| x.ribbon_needed()).sum::<u32>();

    println!("paper needed: {} sq ft", paper_needed);
    println!("ribbon needed: {} ft", ribbon_needed);
}
