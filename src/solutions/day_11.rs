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

    let blinker_guy = 6;

    for n in 0..blinker_guy {
        blink(&mut stones);
    }
    println!("---{}", stones.len());
}

fn blink(stones: &mut Vec<usize>) {
    let mut new_stones: Vec<usize> = Vec::new();
    stones.iter().for_each(|&stone| {
        if stone == 0 {
            new_stones.push(1);
            return;
        };

        let dig = (stone as f64).log(10.0).floor() as usize + 1;
        match dig % 2 {
            0 => {
                let div = 10_usize.pow((dig / 2) as u32);
                new_stones.push(stone / div);
                new_stones.push(stone % div);
            }
            _ => new_stones.push(
                (stone << 10)
                    + (stone << 9)
                    + (stone << 8)
                    + (stone << 7)
                    + (stone << 6)
                    + (stone << 5)
                    + (stone << 3),
            ),
        };
    });

    *stones = new_stones;
}

fn blink02(stones: &mut HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones: HashMap<usize, usize> = HashMap::new();
    for (&key, q) in stones.iter() {
        if key == 0 {
            new_stones.entry(1).and_modify(|x| *x += q).or_insert(*q);
            continue;
        };

        let dig = (key as f64).log(10.0).floor() as u32 + 1;
        match dig % 2 {
            0 => {
                let div = 10_usize.pow((dig / 2) as u32);
                let left = key / div;
                let right = key % div;
                new_stones.entry(left).and_modify(|x| *x += q).or_insert(*q);
                new_stones
                    .entry(right)
                    .and_modify(|x| *x += q)
                    .or_insert(*q);
            }
            _ => {
                let res = (key << 10)
                    + (key << 9)
                    + (key << 8)
                    + (key << 7)
                    + (key << 6)
                    + (key << 5)
                    + (key << 3);

                new_stones.entry(res).and_modify(|x| *x += q).or_insert(*q);
            }
        };
    }

    new_stones
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(11, false);

    let mut real_stones: HashMap<usize, usize> = HashMap::new();
    input
        .first()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .for_each(|s| {
            let num = s.parse::<usize>().unwrap();
            real_stones.entry(num).or_insert(1);
        });

    let blinker_guy = 75;

    for n in 0..blinker_guy {
        let start = Instant::now();
        real_stones = blink02(&mut real_stones);
        let dur = start.elapsed();
        let mut sum: usize = 0;
        real_stones.iter().for_each(|(_, &q)| sum += q as usize);
        println!("B-{} {:?} ", n, dur);
    }

    let mut sum: usize = 0;
    real_stones.iter().for_each(|(_, &q)| sum += q);
    println!("---{}", sum);
}
