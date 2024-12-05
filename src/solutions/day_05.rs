use crate::utils::setup;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_multi_input_lines_vec(5, false);

    let inst_map = get_instruction_map(&input[0]);

    let mut sum = 0;
    input[1].iter().for_each(|u| {
        let pages = u.split(",").collect::<Vec<&str>>();
        let mut is_valid = true;

        check_update(&pages, &inst_map, || is_valid = false);
        if is_valid {
            let center_index = pages.len() / 2;
            sum += pages[center_index].parse::<i32>().unwrap();
        }
    });

    println!("{:#?}", sum);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(5, false);

    println!("{:#?}", input);
}

fn get_instruction_map(input: &Vec<String>) -> HashMap<&str, Vec<&str>> {
    let instructions = input
        .iter()
        .map(|x| {
            let splitted = x.split('|').collect::<Vec<&str>>();
            return (splitted[0], splitted[1]);
        })
        .collect::<Vec<(&str, &str)>>();
    let mut inst_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for (key, value) in instructions {
        inst_map.entry(key).or_insert_with(Vec::new).push(value);
    }

    return inst_map;
}

fn check_update<F>(pages: &Vec<&str>, inst_map: &HashMap<&str, Vec<&str>>, mut on_some: F)
where
    F: FnMut(),
{
    for n in (0..pages.len()).rev() {
        let page_nr = pages[n];
        if !&inst_map.contains_key(&page_nr) {
            continue;
        }

        let instr = &inst_map[page_nr];

        for m in 0..instr.len() {
            let update_chunk = &pages[0..n];
            let search = update_chunk.iter().position(|&p| instr[m] == p);
            if search.is_some() {
                on_some();
                break;
            }
        }
    }
}
