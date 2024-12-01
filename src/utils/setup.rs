use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub enum InputType {
    Test,
    Main,
}

pub fn get_input_lines_vec(day_nr: u8, input_type: InputType) -> Vec<String> {
    let formated_day = format!("{:02}", day_nr);
    let path =
        String::from("src/inputs/") + formated_day.as_str() + "/" + input_type.as_str() + ".txt";

    println!("{}", path);
    let file = File::open(path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    return line_string_vec;
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Test => "test",
            InputType::Main => "main",
        }
    }
}
