use crate::{structs::position_struct::Position, utils::setup};
use std::rc::Rc;
use std::{collections::HashMap, time::Instant};

pub struct MazeRunner {
    direction: Direction,
    end_position: Position,
    path_sign: String,
    maze: Rc<Vec<Vec<String>>>,
    position: Position,
    runners: Vec<Runner>,
}

impl MazeRunner {
    pub fn new(
        matrix: Rc<Vec<Vec<String>>>,
        start_sign: &str,
        end_sign: &str,
        path_sign: &str,
    ) -> Self {
        let mut inst = Self {
            direction: Direction::East,
            end_position: Position::find_position(&matrix, end_sign.to_string())
                .expect("end_sign not found on matrix"),
            path_sign: String::from(path_sign),
            position: Position::find_position(&matrix, start_sign.to_string())
                .expect("start_sign not found on matrix"),
            maze: matrix,
            runners: vec![],
        };
        let runner = Runner {
            direction: inst.direction.clone(),
            end_position: inst.end_position.clone(),
            position: inst.position,
            path_sign: inst.path_sign.clone(),
            maze: Rc::clone(&inst.maze),
            path: Vec::new(),
            points: 0,
        };

        inst.runners.push(runner);

        return inst;
    }

    pub fn run(&mut self, print_winners: bool) -> usize {
        let mut winners: Vec<Runner> = Vec::new();
        let mut new_runners: HashMap<Position, Runner> = HashMap::new();
        let mut smallest = 0;

        let start = Instant::now();

        let mut iter = 0;
        while self.runners.len() > 0 {
            let start = Instant::now();
            self.runners.iter_mut().for_each(|rnr| {
                if rnr.position.x == self.end_position.x && rnr.position.y == self.end_position.y {
                    winners.push(rnr.clone());
                } else {
                    rnr.run().iter().for_each(|nrnr| {
                        let entry = new_runners.entry(nrnr.position).or_insert(nrnr.clone());
                        if entry.points > nrnr.points {
                            *entry = nrnr.clone();
                        }
                    });
                }
            });

            let min_win = new_runners.iter().min_by_key(|&(_, w)| w.points);
            if min_win.is_some() {
                let mw = min_win.unwrap();
                if mw.1.points < smallest {
                    smallest = mw.1.points
                };
            };

            if smallest != 0 && new_runners.iter().all(|x| x.1.points > smallest) {
                break;
            }

            self.runners = new_runners.iter().map(|(_, v)| v.clone()).collect();
            if winners.len() > 0 && smallest != 0 {
                self.runners.retain(|rn| rn.points < smallest);
            }
            new_runners.clear();
            let duration = start.elapsed();
            println!("{:?} {:?} {:?}", iter, duration, self.runners.len());
            iter += 1;
        }
        let duration = start.elapsed();
        println!("{:?} {:?}", duration, self.runners.len());

        if print_winners {
            winners.iter().for_each(|w| {
                println!("{:?}", w.points);
                setup::print_matrix(
                    &self.maze,
                    Some(|y, x, c: String| {
                        if w.path.iter().find(|p| p.x == x && p.y == y).is_some() {
                            print!("•");
                        } else {
                            print!("{}", c);
                        }
                    }),
                );
            });
        }

        winners.sort_by_key(|w| w.points);
        return winners.first().unwrap().points;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
struct Runner {
    direction: Direction,
    end_position: Position,
    path_sign: String,
    path: Vec<Position>,
    maze: Rc<Vec<Vec<String>>>,
    position: Position,
    points: usize,
}

impl Runner {
    fn next(
        &self,
        direction: Direction,
        position: Position,
        points: usize,
        path: Vec<Position>,
    ) -> Self {
        Self {
            direction,
            end_position: self.end_position.clone(),
            path_sign: self.path_sign.clone(),
            maze: Rc::clone(&self.maze),
            path,
            position,
            points,
        }
    }

    fn run(&mut self) -> Vec<Runner> {
        let mut new_runners: Vec<Runner> = Vec::new();
        self.path.push(self.position);

        let calculate_points = |direction: &Direction| {
            self.points
                + if &self.direction == direction {
                    1
                } else {
                    1001
                }
        };

        let go_there = |y: usize, x: usize| {
            self.end_position.x == x && self.end_position.y == y
                || self.maze[y][x] == self.path_sign
                    && self.path.iter().find(|pa| pa.y == y && pa.x == x).is_none()
        };

        let mut check_dir = |y: usize, x: usize, direction: &Direction| {
            if go_there(y, x) {
                new_runners.push(self.next(
                    direction.clone(),
                    Position { x, y },
                    calculate_points(direction),
                    self.path.clone(),
                ));
            };
        };

        check_dir(self.position.y, self.position.x + 1, &Direction::East);
        check_dir(
            self.position.y,
            (self.position.x as i32 - 1) as usize,
            &Direction::West,
        );

        check_dir(self.position.y + 1, self.position.x, &Direction::North);
        check_dir(
            (self.position.y - 1) as usize,
            self.position.x,
            &Direction::South,
        );

        return new_runners;
    }
}
