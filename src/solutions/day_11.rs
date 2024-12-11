use crate::utils::setup;
use std::time::Instant;
use std::collections::HashMap;

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
    stones.iter().for_each(|&stone|  {
        if stone == 0 {
            new_stones.push(1);
            return;
        };

        let dig = (stone as f64).log(10.0).floor() as usize + 1;
        match dig % 2 {
            0 =>  {
                let div = 10_usize.pow((dig / 2) as u32);
                new_stones.push(stone / div);
                new_stones.push(stone % div);
            },
            1 => {
                new_stones.push((stone << 10) +  (stone << 9) + (stone << 8) +(stone << 7) +(stone << 6) + (stone << 5) + (stone << 3))
            }
            _ => {}
        };
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

    for n in 0..blinker_guy  {
        let start = Instant::now();
        blink(&mut stones);
        let dur = start.elapsed();
        println!("{} {:?}", n, dur);
    }

    println!("---{}", stones.len());
}
