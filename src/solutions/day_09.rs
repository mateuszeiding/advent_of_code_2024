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

    for n in (0..blocks.len()).rev() {
        if blocks[n].contains(&".".to_string()) {
            continue;
        }

        let block_len = blocks[n].len();
        let first_dot_pos = blocks
            .iter()
            .position(|x| x.contains(&".".to_string()) && x.len() >= block_len);

        match first_dot_pos {
            Some(dfp) => {
                if n < dfp {
                    continue;
                }

                let diff = blocks[dfp].len().abs_diff(block_len);
                blocks.swap(dfp, n);

                if diff == 0 {
                    continue;
                }

                blocks[n] = blocks[n][0..block_len].to_vec();
                blocks.insert(dfp + 1, vec![".".to_string(); diff]);
            }
            None => continue,
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

    println!(
        "{:?}",
        blocks
            .iter()
            .flatten()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    println!("{} {}", sum, iter);
}
