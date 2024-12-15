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

pub fn get_input_matrix(vec: Vec<String>) -> Vec<Vec<String>> {
    let mut matrix: Vec<Vec<String>> = Vec::new();

    vec.iter().for_each(|x| {
        // Split the string into characters and collect them as String instead of &str
        matrix.push(x.chars().map(|c| c.to_string()).collect::<Vec<String>>());
    });

    matrix
}

#[allow(dead_code)]
pub fn print_matrix<F>(matrix: &Vec<Vec<String>>, matcher: Option<F>)
where
    F: Fn(usize, usize, String),
{
    for (y, row) in matrix.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if let Some(ref m) = matcher {
                m(y, x, matrix[y][x].to_string());
            } else {
                print!("{}", value);
            }
        }
        println!();
    }
    println!("-------------");
}

pub fn get_multi_input_lines_vec(day_nr: u8, test: bool) -> Vec<Vec<String>> {
    let vec = get_input_lines_vec(day_nr, test);
    let mut multi_lines: Vec<Vec<String>> = Vec::new();
    let mut current_line: Vec<String> = Vec::new();
    vec.iter().for_each(|line| {
        if line.trim().is_empty() {
            if !current_line.is_empty() {
                multi_lines.push(current_line.clone());
                current_line = Vec::new();
            }
        } else {
            current_line.push(line.to_string());
        }
    });

    if !current_line.is_empty() {
        multi_lines.push(current_line);
    }
    return multi_lines;
}
