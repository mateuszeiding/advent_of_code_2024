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
    input.iter().for_each(|x| {
        let level_split = x
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        println!("{:?}", x);
        let all_safe = is_safe(&level_split);
        if all_safe.is_none() {
            safe_rep_count += 1;
            return;
        }

        for n in 0..level_split.len() {
            let mut ls = create_level_split(x);
            ls.remove(n);
            let safe = is_safe(&ls);
            if safe.is_none() {
                safe_rep_count += 1;

                break;
            }
        }
    });

    println!("{:#?}", safe_rep_count);
}

fn is_safe(vec: &Vec<i32>) -> Option<usize> {
    let mut diff: Vec<i32> = Vec::new();
    for n in 0..vec.len() - 1 {
        diff.push(vec[n] - vec[n + 1]);
    }
    return diff
        .iter()
        .position(|&x| x.is_positive() != diff[0].is_positive() || x == 0 || x.abs() > 3);
}

fn create_level_split(rep: &String) -> Vec<i32> {
    return rep
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}
