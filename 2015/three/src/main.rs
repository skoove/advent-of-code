use std::fs;

use three::*;

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");

    let santa = Santa::new_from_directions(input.trim().into());

    let houses_viseted_once = santa.houses.iter().filter(|house| *house.1 >= 1);

    println!(
        "Houses viseted at least once: {}",
        houses_viseted_once.count()
    );
}
