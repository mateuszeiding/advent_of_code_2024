use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Robot {
    pos: Position,
    moves: Vec<String>,
}

impl Robot {
    pub fn next_step(&mut self, matrix: &mut Vec<Vec<String>>) {
        let next_move = self.moves.remove(0);
        let pos: (i32, i32) = match next_move.as_str() {
            ">" => (1, 0),
            "<" => (-1, 0),
            "v" => (0, 1),
            "^" => (0, -1),
            _ => panic!("Wrong direction"),
        };

        let x = self.pos.x as i32 + pos.0;
        let y = self.pos.y as i32 + pos.1;

        match matrix[y as usize][x as usize].as_str() {
            "." => {
                matrix[self.pos.y][self.pos.x] = ".".to_string();
                self.pos.x = x as usize;
                self.pos.y = y as usize;
                matrix[self.pos.y][self.pos.x] = "@".to_string();
            }
            "#" => {}
            "O" => {
                let mut shift_size = 1;
                loop {
                    let x = self.pos.x as i32 + (pos.0 * shift_size);
                    let y = self.pos.y as i32 + (pos.1 * shift_size);
                    let next_el = matrix[y as usize][x as usize].as_str();
                    match next_el {
                        "O" => shift_size += 1,
                        "#" => {
                            shift_size = 0;
                            break;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if shift_size == 0 {
                    return;
                }

                (1..shift_size).for_each(|_| {
                    let x = self.pos.x as i32 + (pos.0 * (shift_size));
                    let y = self.pos.y as i32 + (pos.1 * (shift_size));
                    matrix[y as usize][x as usize] = "O".to_string();
                    shift_size -= 1;
                });

                let x = self.pos.x as i32 + pos.0;
                let y = self.pos.y as i32 + pos.1;
                matrix[self.pos.y][self.pos.x] = ".".to_string();

                self.pos.x = x as usize;
                self.pos.y = y as usize;
                matrix[self.pos.y][self.pos.x] = "@".to_string();
            }
            _ => panic!("What I should do?"),
        }
        // setup::print_matrix::<fn(usize, usize, String)>(&matrix, None);
    }

    pub fn work(&mut self, matrix: &mut Vec<Vec<String>>) {
        let range = 0..self.moves.len();
        range.for_each(|_| self.next_step(matrix));
    }

    pub fn next_hard_step(&mut self, matrix: &mut Vec<Vec<String>>) {
        let next_move = self.moves.remove(0);
        let pos: (i32, i32) = match next_move.as_str() {
            ">" => (1, 0),
            "<" => (-1, 0),
            "v" => (0, 1),
            "^" => (0, -1),
            _ => panic!("Wrong direction"),
        };

        let x = self.pos.x as i32 + pos.0;
        let y = self.pos.y as i32 + pos.1;

        let x_shift = |matrix: &mut Vec<Vec<String>>| {
            let mut shift_size = 1;
            let y = self.pos.y;
            loop {
                let x = self.pos.x as i32 + (pos.0 * shift_size);
                match matrix[y][x as usize].as_str() {
                    "[" | "]" => shift_size += 1,
                    "#" => {
                        shift_size = 0;
                        break;
                    }
                    _ => break,
                }
            }

            if shift_size == 0 {
                return self.pos;
            }

            let mut x = self.pos.x as i32 - pos.0 + (pos.0 * (shift_size));

            (0..shift_size).for_each(|_| {
                let s = if pos.0 > 0 {
                    ((x + pos.0) as usize, x as usize)
                } else {
                    (x as usize, (x + pos.0) as usize)
                };

                matrix[y].swap(s.0, s.1);
                x = x - pos.0;

                // setup::print_matrix::<fn(usize, usize, String)>(&matrix, None);
            });

            let x = self.pos.x as i32 + if pos.0 > 0 { 1 } else { -1 };

            matrix[y][self.pos.x] = ".".to_string();
            matrix[y][x as usize] = "@".to_string();
            // setup::print_matrix::<fn(usize, usize, String)>(&matrix, None);

            return Position {
                x: x as usize,
                y: y as usize,
            };
        };

        let y_shift = |matrix: &mut Vec<Vec<String>>| {
            if matrix[(self.pos.y as i32 + pos.1) as usize][self.pos.x].as_str() == "." {
                let x = self.pos.x as i32;
                let y = self.pos.y as i32 + pos.1;

                matrix[self.pos.y][self.pos.x] = ".".to_string();
                matrix[y as usize][x as usize] = "@".to_string();

                return Position {
                    x: x as usize,
                    y: y as usize,
                };
            }

            let x_diff: i32 =
                if matrix[(self.pos.y as i32 + pos.1) as usize][self.pos.x].as_str() == "[" {
                    0
                } else {
                    -1
                };

            let mut shift: Vec<Vec<RangeInclusive<usize>>> = Vec::new();

            shift.push(vec![
                ((self.pos.x as i32 + x_diff) as usize)
                    ..=((self.pos.x as i32 + 1 + x_diff) as usize),
            ]);

            let mut iter = 0;
            while iter < shift.len() {
                let y = self.pos.y as i32 + (pos.1 * shift.len() as i32);
                for ran in shift[iter].clone() {
                    let next_el = &matrix[(y + pos.1) as usize][ran.clone()];
                    println!(
                        "{:?} {:?} {:?}",
                        next_el,
                        ran,
                        &matrix[(y + pos.1) as usize].len()
                    );
                    match next_el {
                        val if val.contains(&"#".to_string()) => {
                            iter += 1;
                            shift.clear();
                            break;
                        }
                        val if val.first().unwrap() == "[" && val.last().unwrap() == "]" => {
                            shift.push(vec![ran.clone()]);
                            iter += 1;
                        }
                        val if val.first().unwrap() == "]" && val.last().unwrap() == "[" => {
                            let range = (ran.start() - 1)..=(ran.end() + 1);
                            if val.len() > 2 {
                                if val[1..2] != vec!["[", "]"] {
                                    shift.push(vec![
                                        *range.start()..=range.start() + 1,
                                        range.end() - 1 as usize..=*range.end(),
                                    ]);
                                }
                            } else {
                                shift.push(vec![range]);
                            }
                            iter += 1;
                        }
                        val if val.first().unwrap() == "]" && val.last().unwrap() == "." => {
                            shift.push(vec![(ran.start() - 1)..=(ran.end() - 1)]);
                            iter += 1;
                        }
                        val if val.first().unwrap() == "[" && val.last().unwrap() == "." => {
                            shift.push(vec![*ran.start()..=(ran.end() - 2)]);
                            iter += 1;
                        }
                        val if val.first().unwrap() == "." && val.last().unwrap() == "[" => {
                            shift.push(vec![(ran.start() + 1)..=(ran.end() + 1)]);
                            iter += 1;
                        }
                        val if val.first().unwrap() == "." && val.last().unwrap() == "]" => {
                            shift.push(vec![(ran.end() - 2)..=*ran.end()]);
                            iter += 1;
                        }

                        _ => {
                            iter += 1;
                        }
                    }
                }
            }

            if shift.len() == 0 {
                return self.pos;
            }

            let mut y = self.pos.y as i32 + (pos.1 * shift.len() as i32);
            let mut prev_range: Vec<RangeInclusive<usize>> = vec![];
            shift.iter().rev().for_each(|range_vec| {
                range_vec.iter().for_each(|range| {
                    range.clone().for_each(|x| {
                        matrix[(y + pos.1) as usize][x] = matrix[(y) as usize][x].to_string();
                    });

                    prev_range.push(range.clone());
                });

                if prev_range.len() != 0 {
                    prev_range.iter().for_each(|r| {
                        r.clone().for_each(|x| {
                            if !range_vec.iter().find(|r| r.contains(&x.clone())).is_none() {
                                matrix[(y) as usize][x] = ".".to_string();
                            }
                        });
                    });
                }
                prev_range.clear();
                y = y - pos.1;
            });

            // setup::print_matrix::<fn(usize, usize, String)>(&matrix, None);
            let x = self.pos.x as i32;
            let y = self.pos.y as i32 + pos.1;

            matrix[y as usize][(x + pos.0) as usize] = ".".to_string();

            matrix[self.pos.y][self.pos.x] = ".".to_string();
            matrix[y as usize][x as usize] = "@".to_string();

            return Position {
                x: x as usize,
                y: y as usize,
            };
        };

        match matrix[y as usize][x as usize].as_str() {
            "." => {
                matrix[self.pos.y][self.pos.x] = ".".to_string();
                self.pos.x = x as usize;
                self.pos.y = y as usize;
                matrix[self.pos.y][self.pos.x] = "@".to_string();
            }
            "#" => {}
            "[" | "]" => match next_move.as_str() {
                "<" | ">" => self.pos = x_shift(matrix),
                _ => self.pos = y_shift(matrix),
            },
            _ => panic!("What I should do?"),
        }
        // println!("{}", next_move);
        // setup::print_matrix::<fn(usize, usize, String)>(&matrix, None);
    }

    pub fn work_hard(&mut self, matrix: &mut Vec<Vec<String>>) {
        let range = 0..self.moves.len();
        range.for_each(|_| {
            self.next_hard_step(matrix);
            // thread::sleep(Duration::from_millis(500));
        });
    }
}

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_multi_input_lines_vec(15, false);
    let mut matrix = setup::get_input_matrix(input[0].clone());
    let pos = Position::find_position(&matrix, "@".to_string());
    let moves = input[1].join("").chars().map(|x| x.to_string()).collect();
    if pos.is_none() {
        panic!("Robot not found");
    }
    let mut robot = Robot {
        pos: pos.unwrap(),
        moves,
    };

    robot.work(&mut matrix);
    let mut sum = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x].as_str() == "O" {
                sum += 100 * y + x;
            }
        }
    }
    println!("sum: {}", sum);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_multi_input_lines_vec(15, false);
    let mat = setup::get_input_matrix(input[0].clone());
    let mut matrix: Vec<Vec<String>> = Vec::new();
    mat.iter().for_each(|row| {
        let mut r: Vec<String> = Vec::new();
        row.iter().for_each(|col| {
            r.extend(match col.as_str() {
                "#" => vec!["#".to_string(), "#".to_string()],
                "O" => vec!["[".to_string(), "]".to_string()],
                "@" => vec!["@".to_string(), ".".to_string()],
                _ => vec![".".to_string(), ".".to_string()],
            })
        });
        matrix.push(r);
    });

    let pos = Position::find_position(&matrix, "@".to_string());
    let moves = input[1].join("").chars().map(|x| x.to_string()).collect();
    if pos.is_none() {
        panic!("Robot not found");
    }
    let mut robot = Robot {
        pos: pos.unwrap(),
        moves,
    };

    robot.work_hard(&mut matrix);

    let mut sum = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if matrix[y][x].as_str() == "[" {
                sum += 100 * y + x;
            }
        }
    }
    println!("sum: {}", sum);
}
