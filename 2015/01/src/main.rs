use std::{fs, time::Instant};

fn main() {
    let start = Instant::now();

    let input = match fs::read_to_string("input") {
        Ok(input) => input,
        Err(e) => {
            panic!("Failed to open file: {e}")
        }
    };

    let mut floor: i32 = 0;

    for char in input.chars() {
        floor += up_or_down(char);
    }

    println!("The correct floor is: {floor}");
    let end = Instant::now();
    println!("Took {} ns", (end - start).as_nanos());
}

fn up_or_down(char: char) -> i32 {
    match char {
        '(' => 1,
        ')' => -1,
        '\n' => 0,
        _ => panic!("Invalid input: {char}, input should either be '(' or ')'"),
    }
}
