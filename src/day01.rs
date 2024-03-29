use crate::utils;
use itertools::Itertools;
use std::{cmp, error, time::Instant};

// Part 1
// lines are how many calories are in a snack
// empty line signifies new bag
// find elf with largest total calories - how many?

pub fn execute(input_type: &str) -> Result<(), Box<dyn error::Error>> {
    let filename = utils::get_input_path(input_type, 1);
    let lines = utils::read_lines(&filename)?;
    part1(lines.clone())?;
    part1_improved(lines.clone())?;
    part1_map(lines.clone())?;
    part2(lines.clone())?;
    Ok(())
}

fn part1(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    println!("part 1:");
    let mut highest = 0;
    let mut bag = 0;
    for line in &lines {
        // println!("line {}: {}", i, line);
        if line.is_empty() {
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

fn part1_improved(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    let mut highest = 0;
    for bag in lines.split(|s| s.is_empty()) {
        let mut bag_total = 0;
        for line in bag {
            bag_total += line.parse::<usize>()?;
        }
        highest = cmp::max(bag_total, highest);
    }
    println!("\thighest: {}", highest);
    let duration = start.elapsed();
    println!("\tTime elapsed: {:?}", duration);
    Ok(())
}

fn part1_map(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    let mut highest = 0;
    for bag in lines.split(|s| s.is_empty()) {
        let bag_total: usize = bag
            .iter()
            .map(|line| line.parse::<usize>())
            .sum::<Result<_, _>>()?;
        highest = cmp::max(bag_total, highest);
    }
    println!("\thighest: {}", highest);
    let duration = start.elapsed();
    println!("\tTime elapsed: {:?}", duration);
    Ok(())
}

fn total_bag(lines: &[String]) -> usize {
    let total = lines
        .iter()
        .map(|line| line.parse::<usize>())
        .sum::<Result<_, _>>();
    match total {
        Ok(t) => t,
        Err(e) => panic!("Error! {}", e),
    }
}

// Part 2
fn part2(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    println!("part 2:");

    let result: usize = lines
        .split(|s| s.is_empty())
        .map(total_bag)
        .sorted()
        .rev()
        .take(3)
        .sum();

    println!("\tresult: {}", result);
    let duration = start.elapsed();
    println!("\ttime elapsed: {:?}", duration);
    Ok(())
}
