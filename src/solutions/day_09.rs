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

        let mut new_val: String = String::new();
        for _ in 0..dig {
            new_val += &(",".to_string() + &sign);
        }
        if new_val.len() != 0 {
            blocks.push(new_val);
        }
    }

    for n in (0..blocks.len()).rev() {
        if !blocks[n].contains(".") {
            let first_dot_pos = blocks.iter().position(|x| {
                x.contains(".") && x.replace(",", "").len() >= blocks[n].replace(",", "").len()
            });

            match first_dot_pos {
                Some(dfp) => {
                    if n > dfp {
                        let diff =
                            blocks[dfp].replace(",", "").len() - blocks[n].replace(",", "").len();
                        blocks.swap(dfp, n);
                        if diff > 0 {
                            blocks[n] = blocks[n][0..blocks[n].len() - diff].to_string();
                            blocks.insert(dfp + 1, ".".to_string().repeat(diff));
                        }
                    }
                }
                None => continue,
            }
        }
    }

    let mut sum: usize = 0;
    let mut iter: usize = 0;
    for n in 0..blocks.len() {
        if blocks[n].contains(".") {
            let wo_comma: Vec<_> = blocks[n]
                .chars()
                .filter(|x| x.to_string() == ".".to_string())
                .collect();
            iter += wo_comma.len();
            continue;
        }
        let chars: Vec<usize> = blocks[n]
            .split(",")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        println!("{:?}", chars);
        for m in 0..chars.len() {
            sum += iter * chars[m];
            iter += 1;
        }
    }

    println!("{} {}", sum, iter);
}
