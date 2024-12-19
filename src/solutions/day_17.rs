use crate::utils::setup;
use regex::Regex;
use std::time::Instant;

struct PC {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    prg: Vec<usize>,
    output: String,
    tick: usize,
}

#[derive(Debug)]
enum Reg {
    A,
    B,
    C,
}

impl PC {
    fn new(a: usize, b: usize, c: usize, prg: &str) -> Self {
        Self {
            reg_a: a,
            reg_b: b,
            reg_c: c,
            prg: prg
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
            output: String::new(),
            tick: 0,
        }
    }

    fn bipbop(&mut self) {
        let prg_check = self
            .prg
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",");

        while self.tick < self.prg.len() - 1 {
            let instr = self.prg[self.tick].clone();
            let op = self.prg[self.tick + 1].clone();
            self.call_instr(&instr, &op);

            if prg_check[0..self.output.len()] != self.output {
                break;
            }
        }

        // println!("{:}", self.get_output());
    }

    fn get_combo_operand(&mut self, operand: &usize) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand.clone(),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("OH NO. I'm a bad boy :<"),
            _ => panic!("I shouldn't exist!"),
        }
    }

    fn call_instr(&mut self, instr: &usize, operand: &usize) {
        match instr {
            0 => self.dv(operand, Reg::A),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.dv(operand, Reg::B),
            7 => self.dv(operand, Reg::C),
            _ => panic!("I shouldn't exist!"),
        }

        if instr != &3 {
            self.tick += 2;
        };
    }

    // opt 0/6/7
    fn dv(&mut self, operand: &usize, reg: Reg) {
        let res = self.reg_a / (1 << self.get_combo_operand(operand));
        match reg {
            Reg::A => self.reg_a = res,
            Reg::B => self.reg_b = res,
            Reg::C => self.reg_c = res,
        };
    }

    // opt 1
    fn bxl(&mut self, operand: &usize) {
        self.reg_b = self.reg_b ^ operand;
    }

    // opt 2
    fn bst(&mut self, operand: &usize) {
        self.reg_b = self.get_combo_operand(operand) & 7;
    }

    // opt 3
    fn jnz(&mut self, operand: &usize) {
        if self.reg_a == 0 {
            self.tick += 2;
        } else {
            self.tick = *operand;
        }
    }

    // opt 4
    // some legacy _operand, does nothing but crashes on prod w/o it
    fn bxc(&mut self, _operand: &usize) {
        self.reg_b ^= self.reg_c;
    }

    // opt 5
    fn out(&mut self, operand: &usize) {
        let co = self.get_combo_operand(operand);
        self.output += &((co & 7).to_string() + ",");
    }
}

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_multi_input_lines_vec(17, false);
    let get_params_regex = Regex::new(r": (\d{1,}.*)").unwrap();
    let get_params = |input: &str| {
        get_params_regex
            .captures(&input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
    };

    let a = get_params(&input[0][0]).parse::<usize>().unwrap();
    let b = get_params(&input[0][1]).parse::<usize>().unwrap();
    let c = get_params(&input[0][2]).parse::<usize>().unwrap();
    let prg = get_params(&input[1][0]);

    let mut pc = PC::new(a, b, c, &prg);
    pc.bipbop();
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_multi_input_lines_vec(17, false);
    let get_params_regex = Regex::new(r": (\d{1,}.*)").unwrap();
    let get_params = |input: &str| {
        get_params_regex
            .captures(&input)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
    };

    // let mut a = get_params(&input[0][0]).parse::<usize>().unwrap();
    let b = get_params(&input[0][1]).parse::<usize>().unwrap();
    let c = get_params(&input[0][2]).parse::<usize>().unwrap();
    let prg = get_params(&input[1][0]);

    let mut pc: PC;
    let mut a = 0;
    loop {
        let start = Instant::now();
        pc = PC::new(a, b, c, &prg);
        pc.bipbop();
        pc.output.pop();

        if pc.output == prg {
            break;
        }
        let dur = start.elapsed();
        println!("{:?} {:}", dur, a);
        a += 1;
    }
    println!("{:?}", a);
}
