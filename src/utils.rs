use std::fs;
use std::io;

// The output is wrapped in a result to allow matching on errors
// Returns an iterator to the Reader of the lines of a file
pub fn read_lines(filename: &str) -> Result<Vec<String>, io::Error> {
    let file = fs::read_to_string(filename)?;
    Ok(file.lines().map(&str::to_string).collect())
}

pub fn get_input_path(input_type: &str, day: u8) -> String {
    let mut day_string = day.to_string();
    if day_string.len() == 1 {
        day_string = format!("0{}", day_string);
    }
    let dir = match input_type {
        "test" => "test_input",
        "real" => "input",
        _ => panic!("Fuck! That's not an input type!")
    };
    format!("{}/day{}.txt", dir, day_string)
}

