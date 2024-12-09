use crate::utils::setup;
use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(7, false);

    let mut instr: Vec<(&str, Vec<usize>)> = Vec::new();
    input.iter().for_each(|x| {
        let key_val = x.split(':').collect::<Vec<&str>>();
        let mut values: Vec<usize> = Vec::new();
        key_val[1]
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|n| {
                let num = n.parse::<usize>();
                if num.is_ok() {
                    values.push(num.unwrap());
                }
            });

        instr.push((key_val[0], values));
    });

    let signs = vec!["+", "*"];
    let mut final_result: usize = 0;

    for (key, val) in instr.clone().into_iter() {
        let signs_cart = get_cartesian(&signs, val.len() - 1);
        for i in 0..signs_cart.len() {
            let result = key.parse::<usize>().unwrap();
            let curr_signs: Vec<String> = signs_cart[i].chars().map(|x| x.to_string()).collect();
            let mut sum: usize = val[0];
            for n in 0..val.len() - 1 {
                match curr_signs[n].as_str() {
                    "+" => {
                        sum += val[n + 1];
                    }

                    "*" => {
                        sum *= val[n + 1];
                    }
                    _ => break,
                }
            }
            if sum > result {
                continue;
            }

            if sum == result {
                final_result += sum;
                break;
            }

        }
    }
    println!("{:#?}", final_result);
}

fn get_cartesian(chars: &Vec<&str>, len: usize) -> Vec<String> {
    let repeated_iters = vec![chars.iter().cloned(); len];

    let combo = repeated_iters
        .into_iter()
        .multi_cartesian_product()
        .map(|x| x.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    combo
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(7, true);

    println!("{:#?}", input);
}
