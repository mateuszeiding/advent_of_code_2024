use crate::structs::position_struct::Position;
use crate::utils::setup;

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

    fn work(&mut self, matrix: &mut Vec<Vec<String>>) {
        let range = 0..self.moves.len();
        range.for_each(|_| self.next_step(matrix));
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
    let input = setup::get_input_lines_vec(2, false);

    println!("{:#?}", input);
}
