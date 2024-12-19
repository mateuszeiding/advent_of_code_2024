use crate::utils::setup;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(9, false)
        .first()
        .unwrap()
        .clone();
    let disc_map: Vec<String> = input.chars().map(|x| x.to_string()).collect();
    let mut blocks: Vec<String> = Vec::new();

    let mut id = 0;
    for n in 0..disc_map.len() {
        let dig = disc_map[n].parse::<usize>().unwrap();
        let sign: String;
        if n % 2 == 0 {
            let num = id.clone();
            id += 1;
            sign = num.to_string();
        } else {
            sign = ".".to_string();
        };

        for _ in 0..dig {
            blocks.push(sign.clone());
        }
    }

    for n in (0..blocks.len()).rev() {
        if blocks[n] != "." {
            let first_dot_pos = blocks.iter().position(|x| x == ".").unwrap();
            if n > first_dot_pos {
                blocks.swap(first_dot_pos, n);
            }
        }
    }
    println!("{:?}", blocks);
    let mut sum: usize = 0;
    for n in 0..blocks.len() {
        if blocks[n] == "." {
            break;
        }
        sum += n * blocks[n].parse::<usize>().unwrap();
    }

    println!("{} {}", sum, blocks.len());
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(9, false)
        .first()
        .unwrap()
        .clone();

    let disc_map: Vec<String> = input.chars().map(|x| x.to_string()).collect();
    let mut blocks: Vec<Vec<String>> = Vec::new();
    for n in 0..disc_map.len() {
        let dig = disc_map[n].parse::<usize>().unwrap();
        let sign = if n % 2 == 0 {
            (n / 2).to_string()
        } else {
            ".".to_string()
        };

        let mut new_val: Vec<String> = Vec::new();
        (0..dig).for_each(|_| new_val.push(sign.clone()));
        if new_val.len() != 0 {
            blocks.push(new_val);
        }
    }

    let dedup_dots = |blocks: &mut Vec<Vec<String>>, iter: &mut usize, addi: &mut usize| {
        // Thank you ChaJPT for this thing
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut dot_group: Vec<String> = Vec::new();

        for vec in blocks.clone() {
            if vec.contains(&".".to_string()) {
                dot_group.extend(vec.clone());
            } else {
                if !dot_group.is_empty() {
                    result.push(dot_group.clone());
                    dot_group.clear();
                }
                result.push(vec.clone());
            }
        }

        if !dot_group.is_empty() {
            result.push(dot_group);
        }
        // Thank you ChaJPT for this thing

        *addi -= blocks.len().abs_diff(result.len());
        *blocks = result.clone();
        *iter = result.len() - 1 - *addi;
    };

    let mut addi = 0;
    let mut iter = blocks.len() - addi;
    while 0 < iter {
        dedup_dots(&mut blocks, &mut iter, &mut addi);
        if blocks[iter].contains(&".".to_string()) {
            addi += 1;
            continue;
        }
        let block_len = blocks[iter].len();
        let available_place = blocks
            .iter()
            .position(|x| x.contains(&".".to_string()) && x.len() >= block_len);

        match available_place {
            Some(p) => {
                if iter < p {
                    addi += 1;
                    continue;
                }

                let diff = blocks[p].len().abs_diff(block_len);
                blocks.swap(p, iter);

                if diff == 0 {
                    addi += 1;
                    continue;
                }

                blocks[iter] = blocks[iter][0..block_len].to_vec();

                blocks.insert(p + 1, vec![".".to_string(); diff]);

                addi += 1;
            }
            None => {
                addi += 1;
                continue;
            }
        }
    }

    let mut sum: usize = 0;
    let mut iter: usize = 0;

    for block in &blocks {
        if block.contains(&".".to_string()) {
            iter += block.len();
            continue;
        }
        let numbers: Vec<usize> = block.iter().map(|x| x.parse::<usize>().unwrap()).collect();

        for n in numbers {
            sum += iter * n;
            iter += 1;
        }
    }

    println!("{:?}", blocks);
    println!("{} {}", sum, iter);
}
