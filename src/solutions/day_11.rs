use crate::utils::setup;

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
    let mut ctr = stones.len();
    let mut i = 0;
    while i < ctr {
        match stones[i] {
            s if s == 0 => stones[i] = 1,
            s if s.to_string().len() % 2 == 0 => {
                let stn = s.to_string();
                stones[i] = stn[0..stn.len() / 2].parse::<usize>().unwrap();
                stones.insert(
                    i + 1,
                    stn[stn.len() / 2..stn.len()].parse::<usize>().unwrap(),
                );
                ctr += 1;
                i += 1;
            }
            s => stones[i] = s * 2024,
        }
        i += 1;
    }
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(2, false);

    println!("{:#?}", input);
}
