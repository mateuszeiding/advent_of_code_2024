use crate::structs::lr_struct;
use crate::utils::setup;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(1, false);
    let input_lr_split = prep_lr_split_vec(&input);
    // dont need struct, doing it for training
    let mut input_structs: Vec<lr_struct::LR> = Vec::new();
    for n in 0..input.len() {
        input_structs.push(lr_struct::LR {
            l: input_lr_split.0[n].clone(),
            r: input_lr_split.1[n].clone(),
        });
    }

    let mut sum = 0;

    input_structs.iter().for_each(|x| {
        let dist = x.r.parse::<i32>().unwrap() - x.l.parse::<i32>().unwrap();
        sum += dist.abs();
    });

    println!("{:#?}", sum);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(1, false);
    let input_lr_split = prep_lr_split_vec(&input);

    let mut sum = 0;
    input_lr_split.0.iter().for_each(|x| {
        let num = x.parse::<i32>().unwrap();
        sum += num * count_nr_quantiti_in_vec(num, &input_lr_split.1);
    });

    println!("{}", sum);
}

fn prep_lr_split_vec(input: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut input_lr_split: (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    input.iter().for_each(|x| {
        let pair = x.split_whitespace().collect::<Vec<&str>>();

        input_lr_split.0.push(pair[0].to_string());
        input_lr_split.1.push(pair[1].to_string());
    });
    input_lr_split.0.sort();
    input_lr_split.1.sort();

    return input_lr_split;
}

// Part 02 priv
fn count_nr_quantiti_in_vec(nr: i32, vec: &Vec<String>) -> i32 {
    let first_occurance = vec.iter().position(|p| p.parse::<i32>().unwrap() == nr);
    if first_occurance.is_none() {
        return 0;
    }
    let mut index = first_occurance.unwrap();
    let mut sum: i32 = 0;

    while vec[index].parse::<i32>().unwrap() == nr {
        sum += 1;
        index += 1;
    }

    return sum;
}
