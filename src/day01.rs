use crate::utils;
use std::{cmp, error, time::Instant};

// Part 1
// lines are how many calories are in a snack
// empty line signifies new bag
// find elf with largest total calories - how many?

pub fn execute(input_type: &str) -> Result<(), Box<dyn error::Error>> {
    let filename = utils::get_input_path(input_type, 1);
    let lines = utils::read_lines(&filename)?;
    part1(lines.clone())?;
    Ok(())
}

fn part1(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    println!("part 1:");
    let mut highest = 0;
    let mut bag = 0;
    for line in &lines {
        // println!("line {}: {}", i, line);
        if line == "" {
            // println!("\tbag total: {}", bag);
            highest = cmp::max(bag, highest);
            bag = 0;
        } else {
            bag += line.parse::<usize>()?;
        }
    }
    // println!("\tbag total: {}", bag);
    highest = cmp::max(bag, highest);
    println!("\thighest: {}", highest);
    let duration = start.elapsed();
    println!("\tTime elapsed: {:?}", duration);
    Ok(())
}
