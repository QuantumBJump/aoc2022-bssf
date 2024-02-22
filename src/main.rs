use std::env;
mod utils;

mod day01;
// mod day02;
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
        // 2 => day01::execute(),
        // 3 => day01::execute(),
        // 4 => day01::execute(),
        // 5 => day01::execute(),
        // 6 => day01::execute(),
        // 7 => day01::execute(),
        // 8 => day01::execute(),
        // 9 => day01::execute(),
        // 10 => day01::execute(),
        // 11 => day01::execute(),
        // 12 => day01::execute(),
        // 13 => day01::execute(),
        // 14 => day01::execute(),
        // 15 => day01::execute(),
        // 16 => day01::execute(),
        // 17 => day01::execute(),
        // 18 => day01::execute(),
        // 19 => day01::execute(),
        // 20 => day01::execute(),
        // 21 => day01::execute(),
        // 22 => day01::execute(),
        // 23 => day01::execute(),
        // 24 => day01::execute(),
        // 25 => day01::execute(),
        _ => panic!("Couldn't execute day {day}!"),
    };
    if let Err(blah) = result {
        panic!("Error! {}", blah);
    }
}

fn determine_day(day_arg: &str) -> u8 {
    day_arg.parse::<u8>().unwrap()
}
