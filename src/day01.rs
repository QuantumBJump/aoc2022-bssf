use std::io;
use crate::utils;

pub fn execute(input_type: &str) -> Result<(), io::Error> {
    let filename = utils::get_input_path(input_type, 1);
    let lines = utils::read_lines(&filename)?;
    Ok(())
}
