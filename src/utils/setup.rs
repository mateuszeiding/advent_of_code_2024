use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn get_input_lines_vec(file_path: String) -> Vec<String> {
    let file = File::open(file_path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    return line_string_vec;
}
