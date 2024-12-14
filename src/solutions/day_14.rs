use crate::utils::setup;
use regex::{Captures, Regex};
use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    velo: Pos,
    max: Pos,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ \"x\": {}, \"y\": \"{}\" }}", self.x, self.y)
    }
}
impl Robot {
    pub fn next(&mut self) {
        self.pos.x = Robot::move_calc(self.pos.x, self.velo.x, self.max.x);
        self.pos.y = Robot::move_calc(self.pos.y, self.velo.y, self.max.y);
    }

    pub fn next_by(&mut self, q: usize) {
        (0..q).for_each(|_| self.next());
    }

    fn move_calc(a: i32, b: i32, max: i32) -> i32 {
        let res = a + b;
        if res < 0 {
            max + res
        } else {
            res % max
        }
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ \"pos\": {}, \"velo\": \"{}\" , \"max\": \"{}\"}}",
            self.pos, self.velo, self.max
        )
    }
}

#[allow(dead_code)]
pub fn part_01() {
    let test = false;
    let input = setup::get_input_lines_vec(14, test);
    let mut robots = get_robots(input, test);
    robots.iter_mut().for_each(|x| x.next_by(100));
    let safe_pos = Pos {
        x: robots[0].max.x / 2,
        y: robots[0].max.y / 2,
    };

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    robots.iter().for_each(|rob| match (rob.pos.x, rob.pos.y) {
        (a, b) if a < safe_pos.x && b < safe_pos.y => q1 += 1,
        (a, b) if a > safe_pos.x && b < safe_pos.y => q2 += 1,
        (a, b) if a < safe_pos.x && b > safe_pos.y => q3 += 1,
        (a, b) if a > safe_pos.x && b > safe_pos.y => q4 += 1,
        _ => {}
    });
    println!("{}", q1 * q2 * q3 * q4);
}

#[allow(dead_code)]
pub fn part_02() {
    let test = false;
    let input = setup::get_input_lines_vec(14, test);
    let mut robots = get_robots(input, test);

    let safe_pos = Pos {
        x: robots[0].max.x / 2,
        y: robots[0].max.y / 2,
    };

    let mut iter = 0;
    loop {
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        robots.iter().for_each(|rob| match (rob.pos.x, rob.pos.y) {
            (a, b) if a < safe_pos.x && b < safe_pos.y => q1 += 1,
            (a, b) if a > safe_pos.x && b < safe_pos.y => q2 += 1,
            (a, b) if a < safe_pos.x && b > safe_pos.y => q3 += 1,
            (a, b) if a > safe_pos.x && b > safe_pos.y => q4 += 1,
            _ => {}
        });
        if q1 > q2 + q3 + q4 || q2 > q1 + q3 + q4 || q3 > q1 + q2 + q4 || q4 > q1 + q2 + q3 {
            for y in 0..robots[0].max.y {
                for x in 0..robots[0].max.x {
                    if robots
                        .iter()
                        .find(|ma| ma.pos.y == y && ma.pos.x == x)
                        .is_some()
                    {
                        print!("%");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!("--------------------");
            println!("{}", iter);
            break;
        }
        iter += 1;
        robots.iter_mut().for_each(|x| x.next());
        robots.sort_by(|a, b| a.pos.x.cmp(&b.pos.x).then_with(|| a.pos.y.cmp(&b.pos.y)));
    }
}

fn get_robots(input: Vec<String>, test: bool) -> Vec<Robot> {
    let max_pos = if test {
        Pos { x: 11, y: 7 }
    } else {
        Pos { x: 101, y: 103 }
    };

    let mut robots: Vec<Robot> = Vec::new();
    let robex = Regex::new(r"p=([+-]?\d{1,},[-+]?\d{1,}).*?v=([+-]?\d{1,},[+-]?\d{1,})").unwrap();
    let get_xy = |cap: &Captures, group: usize| {
        cap.get(group)
            .unwrap()
            .as_str()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    };

    input.iter().for_each(|l| {
        if let Some(cap) = robex.captures(l) {
            let p: Vec<i32> = get_xy(&cap, 1);
            let v: Vec<i32> = get_xy(&cap, 2);
            robots.push(Robot {
                pos: Pos { x: p[0], y: p[1] },
                velo: Pos { x: v[0], y: v[1] },
                max: max_pos.clone(),
            });
        }
    });

    robots
}
