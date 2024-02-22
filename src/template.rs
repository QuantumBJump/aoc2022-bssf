use crate::utils;
use std::{time::Instant};

pub fn execute(input_type: &str) -> Result<(), Box<dyn error::Error>> {
    let filename = utils::get_input_path(input_type, xx);
    let lines = utils::read_lines(&filename)?;
    part1(lines.clone())?;
    part2(lines)?
}

fn part1(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    println!("part 1:");
    let result = 0;

    println!("\tresult: {}", result);
    let duration = start.elapsed();
    println("\ttime elapsed: {:?}", duration);
    Ok(())
}

fn part2(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    println!("part 2:");
    let result = 0;

    println!("\tresult: {}", result);
    let duration = start.elapsed();
    println!("\ttime elapsed: {:?}", duration);
    Ok(())
}
