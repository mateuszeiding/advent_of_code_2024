use crate::structs::lr_struct;
use crate::utils::setup;

pub fn part_01() {
    let input = setup::get_input_lines_vec(1, setup::InputType::Main);
    let mut input_lr_split: (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    input.iter().for_each(|x| {
        let pair = x.split_whitespace().collect::<Vec<&str>>();

        input_lr_split.0.push(pair[0].to_string());
        input_lr_split.1.push(pair[1].to_string());
    });
    input_lr_split.0.sort();
    input_lr_split.1.sort();

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
