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
        println!("{:?}", mach);
        let mut x = 0;
        let mut lowest = 0;
        let a_val = mach.a.x;
        let b_val = mach.b.x;
        let a_bt_b = |x: usize, y: usize| if a_val <= b_val { x } else { y };
        let next_tick = |x: &mut usize| *x += a_bt_b(a_val, b_val);

        for left in (0..mach.prize.x + 1).rev().step_by(a_bt_b(a_val, b_val)) {
            if left % a_bt_b(b_val, a_val) != 0 || x % a_bt_b(a_val, b_val) != 0 {
                next_tick(&mut x);
                continue;
            }

            let b_times = a_bt_b(left, x) / b_val;
            let a_times = a_bt_b(x, left) / a_val;

            println!(
                "{} {}",
                mach.prize.y,
                (mach.a.y * a_times) + (mach.b.y * b_times)
            );
            if mach.prize.y != ((mach.a.y * a_times) + (mach.b.y * b_times)) {
                next_tick(&mut x);
                continue;
            }

            let res = a_times * 3 + b_times;

            println!("letsssgoo {}", res);
            if res < lowest || lowest == 0 {
                lowest = res;
            }
            next_tick(&mut x);
        }

        sum += if lowest != 0 {
            lowest * 10_usize.pow(0)
        } else {
            0
        };
    });

    println!("{:#?} ", sum);
}

fn get_machines(input: &Vec<String>, go_funky: bool) -> Vec<Machine> {
    let reg = Regex::new(r"(X|Y)(=|\+)\d{1,}").unwrap();
    let mut machines: Vec<Machine> = Vec::new();
    let get_vals = |n: usize, gf: bool| {
        let rex = reg
            .find_iter(&input[n])
            .map(|x| {
                let mut num = x.as_str()[2..x.len()].parse::<usize>().unwrap();
                if gf {
                    num += 10_usize.pow(13);
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
        let btn_a = get_vals(n, false);
        let btn_b = get_vals(n + 1, false);
        let prize = get_vals(n + 2, go_funky);

        machines.push(Machine {
            a: btn_a,
            b: btn_b,
            prize,
        });
    }

    machines
}
