use crate::utils::setup;
use regex::Regex;

#[derive(Debug)]
struct XY {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Machine {
    prize: XY,
    a: XY,
    b: XY,
}

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(13, false);
    let machines = get_machines(&input, false);

    let mut sum = 0;

    machines.iter().for_each(|mach| {
        let mut left = mach.prize.x.clone();
        let mut x = 0;
        let mut lowest = 0;
        let a_val = mach.a.x;
        let b_val = mach.b.x;
        let next_tick = |left: &mut usize, x: &mut usize| {
            *left -= if &b_val > left { *left } else { b_val };
            *x += b_val;
        };

        while left != 0 {
            if left % a_val != 0 {
                next_tick(&mut left, &mut x);
                continue;
            }

            let b_times = x / b_val;
            let a_times = left / a_val;

            if !(a_times <= 100
                && b_times <= 100
                && mach.prize.y == (mach.a.y * a_times) + (mach.b.y * b_times))
            {
                next_tick(&mut left, &mut x);
                continue;
            }

            let res = a_times * 3 + b_times;
            if res < lowest || lowest == 0 {
                lowest = res;
            }
            next_tick(&mut left, &mut x);
        }

        sum += if lowest != 0 { lowest } else { 0 };
    });

    println!("{:#?} ", sum);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(13, true);

    let machines = get_machines(&input, true);

    let mut sum = 0;

    machines.iter().for_each(|mach| {
        let mut left = mach.prize.x.clone();
        let mut x = 0;
        let mut lowest = 0;
        let a_val = mach.a.x;
        let b_val = mach.b.x;
        let next_tick = |left: &mut usize, x: &mut usize| {
            *left -= if &b_val > left { *left } else { b_val };
            *x += b_val;
        };

        while left != 0 {
            if left % a_val != 0 {
                next_tick(&mut left, &mut x);
                continue;
            }

            let b_times = x / b_val;
            let a_times = left / a_val;

            if !(mach.prize.y == (mach.a.y * a_times) + (mach.b.y * b_times)) {
                next_tick(&mut left, &mut x);
                continue;
            }

            let res = a_times * 3 + b_times;
            if res < lowest || lowest == 0 {
                lowest = res;
            }
            next_tick(&mut left, &mut x);
        }

        sum += if lowest != 0 { lowest } else { 0 };
    });

    println!("{:#?} ", sum);
}

fn get_machines(input: &Vec<String>, go_funky: bool) -> Vec<Machine> {
    let reg = Regex::new(r"(X|Y)(=|\+)\d{1,}").unwrap();
    let mut machines: Vec<Machine> = Vec::new();
    let get_vals = |n: usize| {
        let rex = reg
            .find_iter(&input[n])
            .map(|x| {
                let mut num = x.as_str()[2..x.len()].parse::<usize>().unwrap();
                if go_funky {
                    num += 10000000000000;
                }
                num
            })
            .collect::<Vec<usize>>();
        XY {
            x: rex[0],
            y: rex[1],
        }
    };

    for n in (0..input.len()).step_by(4) {
        let btn_a = get_vals(n);
        let btn_b = get_vals(n + 1);
        let prize = get_vals(n + 2);

        machines.push(Machine {
            a: btn_a,
            b: btn_b,
            prize,
        });
    }

    machines
}
