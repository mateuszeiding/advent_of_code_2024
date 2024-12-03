use crate::utils::setup;
use regex::Regex;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(3, false);
    let re = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)").unwrap();
    let mut mul_vec: Vec<&str> = Vec::new();
    input.iter().for_each(|x| {
        let mut mul_match = re.find_iter(x).map(|m| m.as_str()).collect::<Vec<&str>>();
        mul_vec.append(&mut mul_match);
    });
    let mut res = 0;
    mul_vec.iter().for_each(|x| {
        let splited = x.split([',', '(', ')']).into_iter().collect::<Vec<&str>>();
        res += splited[1].parse::<i32>().unwrap() * splited[2].parse::<i32>().unwrap();
    });
    println!("{}", res);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(3, false);
    let re = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut mul_vec: Vec<&str> = Vec::new();
    input.iter().for_each(|x| {
        let mut mul_match = re.find_iter(x).map(|m| m.as_str()).collect::<Vec<&str>>();
        mul_vec.append(&mut mul_match);
    });

    println!("{:?}", mul_vec);
    let mut res = 0;
    let mut should_count = true;
    mul_vec.iter().for_each(|x| match x {
        &"do()" => should_count = true,
        &"don't()" => should_count = false,
        _ => {
            if should_count {
                let splited = x.split([',', '(', ')']).into_iter().collect::<Vec<&str>>();
                res += splited[1].parse::<i32>().unwrap() * splited[2].parse::<i32>().unwrap();
            }
        }
    });
    println!("{}", res);
}
