use std::env;
mod utils;

mod day01;
mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (day_arg, input_arg) = match &args[..] {
        [] => panic!("Too few arguments!"),
        [_] => ("01", "real"),
        [_, day] => (day.as_str(), "real"),
        [_, day, input] => (day.as_str(), input.as_str()),
        [_, _, _, ..] => panic!("Too many arguments!"),
    };
    if input_arg != "real" && input_arg != "test" {
        panic!("Invalid input type {}! real or test required", input_arg);
    }

    let day: u8 = determine_day(day_arg);
    println!("Day {}, data: {}", day, input_arg);
    let result = match day {
        1 => day01::execute(input_arg),
        2 => day02::execute(input_arg),
        // 3 => day03::execute(input_arg),
        // 4 => day04::execute(input_arg),
        // 5 => day05::execute(input_arg),
        // 6 => day06::execute(input_arg),
        // 7 => day07::execute(input_arg),
        // 8 => day08::execute(input_arg),
        // 9 => day09::execute(input_arg),
        // 10 => day10::execute(input_arg),
        // 11 => day11::execute(input_arg),
        // 12 => day12::execute(input_arg),
        // 13 => day13::execute(input_arg),
        // 14 => day14::execute(input_arg),
        // 15 => day15::execute(input_arg),
        // 16 => day16::execute(input_arg),
        // 17 => day17::execute(input_arg),
        // 18 => day18::execute(input_arg),
        // 19 => day19::execute(input_arg),
        // 20 => day20::execute(input_arg),
        // 21 => day21::execute(input_arg),
        // 22 => day22::execute(input_arg),
        // 23 => day23::execute(input_arg),
        // 24 => day24::execute(input_arg),
        // 25 => day25::execute(input_arg),
        _ => panic!("day {day} does not exist (yet)!"),
    };
    if let Err(blah) = result {
        panic!("Error! {}", blah);
    }
}

fn determine_day(day_arg: &str) -> u8 {
    match day_arg.parse::<u8>() {
        Ok(res) => res,
        Err(e) => panic!("Failed to parse {day_arg}: {e}"),
    }
}
