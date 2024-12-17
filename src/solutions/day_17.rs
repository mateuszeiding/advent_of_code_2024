use crate::utils::setup;
use regex::Regex;

struct PC {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    prg: Vec<usize>,
    output: Vec<usize>,
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
            output: Vec::new(),
            tick: 0,
        }
    }

    fn bipbop(&mut self) {
        while self.tick < self.prg.len() - 1 {
            self.call_instr(self.prg[self.tick], self.prg[self.tick + 1]);
        }
        println!("{:}", self.get_output());
    }

    fn get_combo_operand(&mut self, operand: usize) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("OH NO. I'm a bad boy :<"),
            _ => panic!("I shouldn't exist!"),
        }
    }

    fn call_instr(&mut self, instr: usize, operand: usize) {
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

        if instr != 3 {
            self.tick += 2;
        };
    }

    // opt 0/6/7
    fn dv(&mut self, operand: usize, reg: Reg) {
        println!("opt 0/6/7 {:} {:?}", operand, reg);
        let res = self.reg_a / 2usize.pow(self.get_combo_operand(operand) as u32);
        match reg {
            Reg::A => self.reg_a = res,
            Reg::B => self.reg_b = res,
            Reg::C => self.reg_c = res,
        };
    }

    // opt 1
    fn bxl(&mut self, operand: usize) {
        println!("opt 1 {:} {:}", self.reg_b, operand);
        self.reg_b = self.reg_b ^ operand;
    }

    // opt 2
    fn bst(&mut self, operand: usize) {
        let res = self.get_combo_operand(operand) % 8;
        let u3 = res & 0b00000111;
        println!(
            "opt 2 {} {:b} {:b} {} {:}",
            res,
            res,
            u3,
            u3,
            self.get_combo_operand(operand)
        );
        self.reg_b = u3;
    }

    // opt 3
    fn jnz(&mut self, operand: usize) {
        println!("opt 3 {:}", operand);
        if self.reg_a == 0 {
            self.tick += 2;
            return;
        }
        self.tick = operand;
    }

    // opt 4
    // some legacy _operand, does nothing but crashes on prod w/o it
    fn bxc(&mut self, _operand: usize) {
        println!("opt 4 {:} {:}", self.reg_b, self.reg_c);
        self.reg_b = self.reg_b ^ self.reg_c;
        println!("opt 4 {:}", self.reg_b);
    }

    // opt 5
    fn out(&mut self, operand: usize) {
        let co = self.get_combo_operand(operand);
        self.output.push(co % 8);
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
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
    println!("{:} {:} {:}", pc.reg_a, pc.reg_b, pc.reg_c);
    println!("{:?}", pc.output.iter().sum::<usize>());
    println!(
        "{:?}",
        pc.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(17, true);

    println!("{:#?}", input);
}
