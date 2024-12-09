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

    let mut sum: usize = 0;
    for n in 0..blocks.len() {
        if blocks[n] == "." {
            break;
        }
        sum += n * blocks[n].parse::<usize>().unwrap();
    }

    println!("{:#?}", sum);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(9, false);

    println!("{:#?}", input);
}

