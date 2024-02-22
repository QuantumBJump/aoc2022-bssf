use crate::utils;
use std::{error, time::Instant};

pub fn execute(input_type: &str) -> Result<(), Box<dyn error::Error>> {
    let filename = utils::get_input_path(input_type, 2);
    println!("{filename}");
    let lines = utils::read_lines(&filename)?;
    part1(lines.clone())?;
    part2(lines)?;
    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
struct Round {
    them: Choice,
    you: Choice,
}

impl Round {
    fn score(&self) -> usize {
        let mut res = 0;
        res += match self.you {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
        res += match self.outcome() {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
        res
    }

    fn outcome(&self) -> Outcome {
        match (&self.you, &self.them) {
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => Outcome::Loss,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (_, _) => Outcome::Draw,
        }
    }
}

// Part 1
// rock paper scissors strategy guide
// first column: ABC for rock paper scissors
// second column: your choice: XYZ for rock paper scissors
// rock beaten by paper beaten by scissors beaten by rock
// score for a round:
//  score for what YOU CHOSE 1 for rock, 2 for paper, 3 for scissors
//  plus
//  score for outcome of round: 0 for loss, 3 for draw, 6 for win
fn part1(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    println!("part 1:");

    let result: usize = lines.iter().map(|l| calculate_round(l)).sum();

    println!("\tresult: {result}");
    let duration = start.elapsed();
    println!("\ttime elapsed: {:?}", duration);
    Ok(())
}

fn calculate_round(round: &str) -> usize {
    let (them, you) = round.split_once(' ').unwrap();
    let them = convert(them);
    let you = convert(you);
    let round = &Round { them, you };
    // println!("{round:?}, outcome: {}", round.score());
    round.score()
}

fn convert(choice: &str) -> Choice {
    match choice {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissors,
        _ => panic!("Unexpected choice: {choice}"),
    }
}

// Part 2
// X loss, Y draw, Z win

fn calculate_play(round: &str) -> usize {
    let (them, you) = round.split_once(' ').unwrap();
    let them = convert(them);
    // println!(" they chose: {them:?}");
    let desired = get_desired_outcome(you);
    // println!(" you want: {desired:?}");
    let round = plan_round(&them, desired);

    round.score()
}

fn plan_round(choice: &Choice, outcome: Outcome) -> Round {
    let you: Choice = match (choice, outcome) {
        (c, Outcome::Draw) => c.clone(), // Copy if we want to draw
        (c, Outcome::Win) => match c {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        },
        (c, Outcome::Loss) => match c {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        },
    };
    // println!("You choose: {you:?}");
    let them = choice.clone();
    Round { them, you }
}

fn get_desired_outcome(input: &str) -> Outcome {
    match input {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unexpected choice: {input}"),
    }
}

fn part2(lines: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    let start = Instant::now();
    println!("part 2:");
    let result: usize = lines.iter().map(|l| calculate_play(l)).sum();

    println!("\tresult: {}", result);
    let duration = start.elapsed();
    println!("\ttime elapsed: {:?}", duration);
    Ok(())
}
