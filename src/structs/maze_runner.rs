use crate::{structs::position_struct::Position, utils::setup};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

pub struct MazeRunner {
    direction: Direction,
    end_position: Rc<Position>,
    path_sign: Rc<String>,
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
            end_position: Rc::new(
                Position::find_position(&matrix, end_sign.to_string())
                    .expect("end_sign not found on matrix"),
            ),
            path_sign: Rc::new(path_sign.to_string()),
            position: Position::find_position(&matrix, start_sign.to_string())
                .expect("start_sign not found on matrix"),
            maze: matrix,
            runners: vec![],
        };
        let runner = Runner {
            direction: inst.direction.clone(),
            end_position: Rc::clone(&inst.end_position),
            position: inst.position,
            path_sign: Rc::clone(&inst.path_sign),
            maze: Rc::clone(&inst.maze),
            path: Vec::new(),
            points: 0,
        };

        inst.runners.push(runner);

        return inst;
    }

    pub fn run(&mut self, print_winners: bool) -> Vec<Runner> {
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
                        } else if entry.points == nrnr.points {
                            setup::print_matrix(
                                &self.maze,
                                Some(|y, x, c: String| {
                                    if entry.path.iter().find(|p| p.x == x && p.y == y).is_some() {
                                        print!("•");
                                    } else {
                                        print!("{}", c);
                                    }
                                }),
                            );

                            setup::print_matrix(
                                &self.maze,
                                Some(|y, x, c: String| {
                                    if nrnr.path.iter().find(|p| p.x == x && p.y == y).is_some() {
                                        print!("•");
                                    } else {
                                        print!("{}", c);
                                    }
                                }),
                            );

                            println!("====vvvvvvvvvvvvvvvvvvvvvvv======================");
                            entry.path.extend(nrnr.path.clone());
                            entry
                                .path
                                .sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
                            entry.path.dedup();
                        }
                    });
                }
            });

            println!("==========================");
            let min_win = winners.iter().min_by_key(|&w| w.points);
            if min_win.is_some() {
                let mw = min_win.unwrap();
                if mw.points < smallest || smallest == 0 {
                    smallest = mw.points
                };
            };

            if smallest != 0 && new_runners.iter().all(|x| x.1.points > smallest) {
                break;
            }

            self.runners = new_runners.iter().map(|v| v.1.clone()).collect();
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

        winners.sort_by_key(|w| w.points);
        let w: Vec<_> = winners
            .iter()
            .filter(|&x| x.points == smallest)
            .map(|x| x.clone())
            .collect();

        if print_winners {
            w.iter().for_each(|w| {
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

        w
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
pub struct Runner {
    direction: Direction,
    end_position: Rc<Position>,
    path_sign: Rc<String>,
    pub path: Vec<Position>,
    maze: Rc<Vec<Vec<String>>>,
    position: Position,
    pub points: usize,
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
            end_position: Rc::clone(&self.end_position),
            path_sign: Rc::clone(&self.path_sign),
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
                || self.maze[y][x].as_bytes() == self.path_sign.as_bytes()
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
