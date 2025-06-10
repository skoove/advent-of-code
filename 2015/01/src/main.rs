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
    let mut santa_entered_basement: bool = false;

    for (position, char) in input.chars().enumerate() {
        floor += up_or_down(char);
        if santa_entered_basement {
            continue;
        }
        if floor == -1 {
            println!(
                "Santa first enters the basement at position {}",
                position + 1
            );
            santa_entered_basement = true;
        }
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
