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

    pub fn shortest(&mut self, print_winners: bool) -> Option<Runner> {
        let mut winner: Option<Runner> = None;
        let mut new_runners: HashMap<Position, Runner> = HashMap::new();

        while self.runners.len() > 0 {
            self.runners.iter_mut().for_each(|rnr| {
                if rnr.position.x == self.end_position.x && rnr.position.y == self.end_position.y {
                    winner = Some(rnr.clone());
                } else {
                    rnr.run().iter().for_each(|r| {
                        let entry = new_runners.entry(r.position).or_insert(r.clone());
                        if entry.path.len() > r.path.len() {
                            *entry = r.clone();
                        }
                    });
                }
            });

            if winner.is_some() {
                break;
            }
            self.runners = new_runners
                .iter()
                .map(|(_, v)| v.clone())
                .filter(|nr| {
                    !new_runners
                        .iter()
                        .any(|any| any.1.path.contains(&nr.position))
                })
                .map(|v| v.clone())
                .collect();

            new_runners.clear();
        }

        if print_winners && winner.is_some() {
            let w = winner.clone().unwrap();
            println!("{:?}", w.path.len());
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
        }

        winner
    }

    pub fn run(&mut self, print_winners: bool) -> Vec<Runner> {
        let mut winners: Vec<Runner> = Vec::new();
        let mut new_runners: Vec<Runner> = Vec::new();

        while self.runners.len() > 0 {
            let start = Instant::now();
            self.runners.iter_mut().for_each(|rnr| {
                if rnr.position.x == self.end_position.x && rnr.position.y == self.end_position.y {
                    winners.push(rnr.clone());
                } else {
                    new_runners.extend(rnr.run());
                }
            });

            new_runners.sort_by(|a, b| a.points.cmp(&b.points));

            self.runners = new_runners
                .iter()
                .filter(|nr| {
                    !new_runners
                        .iter()
                        .any(|any| any.path.contains(&nr.position) && any.points < nr.points)
                })
                .map(|v| v.clone())
                .collect();

            let dur = start.elapsed();
            println!("{:?} {:}", dur, self.runners.len());
            new_runners.clear();
        }

        let sm = winners.iter().map(|p| p.points).min().unwrap();
        winners.sort_by_key(|w| w.points);
        let w: Vec<_> = winners
            .iter()
            .filter(|&x| x.points == sm)
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
