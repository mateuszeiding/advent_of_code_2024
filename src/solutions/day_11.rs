use crate::utils::setup;
use std::collections::HashMap;
use std::time::Instant;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(11, false);

    let mut stones: Vec<usize> = input
        .first()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let blinker_guy = 25;

    for n in 0..blinker_guy {
        blink(&mut stones);

        println!("{}", n);
    }
    println!("---{}", stones.len());
}

fn blink(stones: &mut Vec<usize>) {
    let mut new_stones: Vec<usize> = Vec::new();
    stones.iter().for_each(|stone| match stone {
        s if s == &0 => new_stones.push(1),
        s if s.to_string().len() % 2 == 0 => {
            let stn = s.to_string();
            new_stones.push(stn[0..stn.len() / 2].parse::<usize>().unwrap());
            new_stones.push(stn[stn.len() / 2..stn.len()].parse::<usize>().unwrap());
        }
        s => new_stones.push(s * 2024),
    });

    *stones = new_stones;
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(11, false);

    let mut stones: Vec<usize> = input
        .first()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let blinker_guy = 75;

    for n in 0..blinker_guy {
        let start = Instant::now();
        blink(&mut stones);
        let dur = start.elapsed();
        println!("{} {:?}", n, dur);
    }
    println!("---{}", stones.len());
}
