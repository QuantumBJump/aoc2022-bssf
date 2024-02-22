use std::io;
use crate::utils;

// Part 1
// lines are how many calories are in a snack
// empty line signifies new bag
// find elf with largest total calories - how many?

pub fn execute(input_type: &str) -> Result<(), io::Error> {
    let filename = utils::get_input_path(input_type, 1);
    let lines = utils::read_lines(&filename)?;
    for (i, line) in lines.iter().enumerate() {
        println!("line {}: {}", i, line);
    }
    Ok(())
}
