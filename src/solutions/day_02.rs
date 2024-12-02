use crate::utils::setup;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(2, false);

    let mut safe_rep_count = 0;
    input.iter().for_each(|x| {
        let mut diff: Vec<i32> = Vec::new();

        let level_split = x
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for n in 0..level_split.len() - 1 {
            diff.push(level_split[n] - level_split[n + 1]);
        }
        diff.sort();

        diff.dedup();
        if diff.contains(&0) {
            return;
        }

        if diff[0].abs() > 3 {
            return;
        }

        let is_positive = diff[0].is_positive();
        let mut its_allright_bebe = true;
        for n in 1..diff.len() {
            if diff[n].is_positive() != is_positive || diff[n].abs() > 3 {
                its_allright_bebe = false;
                break;
            };
        }

        if its_allright_bebe {
            safe_rep_count += 1;
        }
    });

    println!("{:#?}", safe_rep_count);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(2, false);

    let mut safe_rep_count = 0;
    input.iter().for_each(|x| {});
    println!("{:#?}", safe_rep_count);
}
