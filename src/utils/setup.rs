use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn get_input_lines_vec(day_nr: u8, test: bool) -> Vec<String> {
    let formated_day = format!("{:02}", day_nr);
    let path = format!(
        "src/inputs/{fd}{pre_ex}txt",
        fd = formated_day.as_str(),
        pre_ex = if test { ".test." } else { "." },
    );

    let file = File::open(path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);
    let lines = reader.lines().filter(|line| line.is_ok());
    let line_string_vec = lines.map(|line| line.unwrap()).collect::<Vec<String>>();

    return line_string_vec;
}
