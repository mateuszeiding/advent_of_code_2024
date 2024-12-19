use crate::utils::setup;
use std::collections::HashMap;

struct TowelGenerator {
    available_towels: Vec<String>,
    possible_towels: usize,
}

impl TowelGenerator {
    pub fn new(available_towels: Vec<String>) -> Self {
        Self {
            available_towels,
            possible_towels: 0,
        }
    }

    pub fn find_all_possible_combinations(&mut self, towel: &str) {
        let mut towel_combo: HashMap<usize, usize> = HashMap::new();

        for n in 0..=towel.len() {
            let new = TowelGenerator::get_next_chunks(&self.available_towels, &towel[0..n], towel);
            new.iter().for_each(|n| {
                let len = n.len();
                let entry = towel_combo.entry(len).or_insert(0);
                *entry += 1;
            });
        }

        let mut keys: Vec<_> = towel_combo.keys().collect();
        keys.sort();

        let sum = towel_combo.iter().map(|(_, v)| v);
        let mut s = 1;
        for (i, v) in &towel_combo {
            s *= v * i;
        }
        println!("{:?}", s);
        // .filter(|&v| v != &1)

        self.possible_towels += sum.sum::<usize>();
    }

    pub fn check_towel_is_possible(&mut self, towel: &str) -> bool {
        let mut test_towels: Vec<String> = Vec::new();

        self.available_towels.iter().for_each(|at| {
            if *at == towel[0..at.len()] {
                test_towels.push(at.to_string())
            }
        });

        while test_towels.len() > 0 {
            let mut n_tt: HashMap<usize, String> = HashMap::new();
            test_towels.iter().for_each(|tt| {
                let new = TowelGenerator::get_next_chunks(&self.available_towels, tt, towel);
                new.iter().for_each(|n| {
                    let len = n.len();
                    if !n_tt.contains_key(&len) {
                        n_tt.insert(len, n.to_string());
                    }
                });
            });

            if n_tt.iter().find(|&(_, tt)| tt == towel).is_some() {
                self.possible_towels += 1;
                return true;
            } else {
                test_towels.clear();
                test_towels = n_tt.iter().map(|(_, v)| v.clone()).collect::<Vec<String>>();
            }
        }

        false
    }

    fn get_next_chunks(at: &Vec<String>, test: &str, towel: &str) -> Vec<String> {
        let mut fit = Vec::new();
        for t in at {
            if test.len() + t.len() <= towel.len() && *t == towel[test.len()..test.len() + t.len()]
            {
                fit.push(test.to_string() + t);
            }
        }

        fit
    }
}

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_multi_input_lines_vec(19, false);
    let at = input[0][0]
        .split(",")
        .map(|x| x.trim().to_string())
        .collect();

    let mut tg = TowelGenerator::new(at);
    input[1].iter().for_each(|t| {
        println!("{:?}", &t);
        tg.check_towel_is_possible(t);
    });
    println!("{:#?}", tg.possible_towels);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_multi_input_lines_vec(19, true);
    let at = input[0][0]
        .split(",")
        .map(|x| x.trim().to_string())
        .collect();

    let mut tg = TowelGenerator::new(at);
    let mut pt = 0;
    input[1].iter().for_each(|t| {
        println!("{:?}", &t);
        if tg.check_towel_is_possible(t) {
            pt += 1;
            tg.find_all_possible_combinations(t);
        }
    });
    println!(
        "{:#?} {:} {:?}",
        tg.possible_towels,
        pt,
        tg.possible_towels - pt
    );
}
