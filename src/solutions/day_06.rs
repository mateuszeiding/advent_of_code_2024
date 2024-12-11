use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Directions {
    North,
    East,
    South,
    West,
}

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(6, false);
    let matrix = setup::get_input_matrix(input);
    let mut curr_position: Position = Position { x: 0, y: 0 };
    let mut obstacles_y: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut obstacles_x: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let max_pos: Position = Position {
        x: matrix[0].len(),
        y: matrix.len(),
    };

    for n in 0..matrix.len() {
        for (i, el) in matrix[n].iter().enumerate() {
            match el.as_str() {
                "#" => {
                    obstacles_y
                        .entry(n.to_string())
                        .or_insert_with(Vec::new)
                        .push(i);

                    obstacles_x
                        .entry(i.to_string())
                        .or_insert_with(Vec::new)
                        .push(n)
                }
                "^" => {
                    curr_position.x = i;
                    curr_position.y = n;
                }
                _ => continue,
            }
        }
    }

    for o in obstacles_y.values_mut() {
        o.sort();
    }

    for o in obstacles_x.values_mut() {
        o.sort();
    }

    let mut path: HashMap<String, Vec<Range<usize>>> = HashMap::new();
    let mut curr_dir = Directions::North;
    fn map_y_ranges(
        hm: &mut HashMap<String, Vec<Range<usize>>>,
        from_to: Range<usize>,
        curr_x: usize,
    ) {
        for n in from_to {
            hm.entry(n.to_string())
                .or_insert_with(Vec::new)
                .push(Range {
                    start: curr_x,
                    end: curr_x + 1,
                });
        }
    }

    while curr_position.x != 0
        && curr_position.x != max_pos.x
        && curr_position.y != 0
        && curr_position.y != max_pos.y
    {
        match curr_dir {
            Directions::North => {
                let obstacle = obstacles_x[&curr_position.x.to_string()]
                    .iter()
                    .rev()
                    .find(|&&y| y < curr_position.y);

                match obstacle {
                    Some(o) => {
                        map_y_ranges(&mut path, *o + 1..curr_position.y, curr_position.x);
                        curr_position.y = o + 1;
                        curr_dir = Directions::East;
                    }
                    None => {
                        for n in 0..curr_position.y + 1 {
                            path.entry(n.to_string())
                                .or_insert_with(Vec::new)
                                .push(Range {
                                    start: curr_position.x,
                                    end: curr_position.x + 1,
                                });
                        }
                        curr_position.y = 0;
                    }
                }
            }
            Directions::South => {
                let obstacle = obstacles_x[&curr_position.x.to_string()]
                    .iter()
                    .find(|&&y| y > curr_position.y);

                match obstacle {
                    Some(o) => {
                        map_y_ranges(&mut path, curr_position.y..*o, curr_position.x);

                        curr_position.y = o - 1;
                        curr_dir = Directions::West;
                    }
                    None => {
                        for n in curr_position.y..max_pos.y {
                            path.entry(n.to_string())
                                .or_insert_with(Vec::new)
                                .push(Range {
                                    start: curr_position.x,
                                    end: curr_position.x + 1,
                                });
                        }
                        curr_position.y = max_pos.y;
                    }
                }
            }
            Directions::East => {
                let obstacle = obstacles_y[&curr_position.y.to_string()]
                    .iter()
                    .find(|&&x| x > curr_position.x);

                match obstacle {
                    Some(o) => {
                        path.entry(curr_position.y.to_string())
                            .or_insert_with(Vec::new)
                            .push(Range {
                                start: curr_position.x,
                                end: *o - 1,
                            });

                        curr_position.x = o - 1;
                        curr_dir = Directions::South;
                    }
                    None => {
                        path.entry(curr_position.x.to_string())
                            .or_insert_with(Vec::new)
                            .push(Range {
                                start: curr_position.x,
                                end: max_pos.x,
                            });
                        curr_position.x = max_pos.x;
                    }
                }
            }
            Directions::West => {
                let obstacle = obstacles_y[&curr_position.y.to_string()]
                    .iter()
                    .rev()
                    .find(|&&x| x < curr_position.x);

                match obstacle {
                    Some(o) => {
                        path.entry(curr_position.y.to_string())
                            .or_insert_with(Vec::new)
                            .push(Range {
                                start: *o + 1,
                                end: curr_position.x,
                            });

                        curr_position.x = o + 1;
                        curr_dir = Directions::North;
                    }
                    None => {
                        path.entry(curr_position.x.to_string())
                            .or_insert_with(Vec::new)
                            .push(Range {
                                start: 0,
                                end: curr_position.x,
                            });

                        curr_position.x = 0;
                    }
                }
            }
        }
    }
    for ranges in path.values_mut() {
        *ranges = merge_ranges(ranges);
    }
    let mut sum = 0;
    path.values().for_each(|r| {
        let mut res = 0;
        r.iter().for_each(|x| res += x.len());
        sum += res;
    });
}

fn merge_ranges(ranges: &mut Vec<Range<usize>>) -> Vec<Range<usize>> {
    ranges.sort_by_key(|r| r.start);
    let mut merged: Vec<Range<usize>> = Vec::new();

    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if last.end >= range.start {
                last.end = last.end.max(range.end);
            } else {
                merged.push(range.clone());
            }
        } else {
            merged.push(range.clone());
        }
    }

    merged
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(6, true);
    let matrix = setup::get_input_matrix(input);
    let mut curr_position: Position = Position { x: 0, y: 0 };
    let mut obstacles_y: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut obstacles_x: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let max_pos: Position = Position {
        x: matrix[0].len().clone() - 1,
        y: matrix.len().clone() - 1,
    };

    for n in 0..matrix.len() {
        for (i, el) in matrix[n].iter().enumerate() {
            match el.as_str() {
                "#" => {
                    obstacles_y
                        .entry(n.to_string())
                        .or_insert_with(Vec::new)
                        .push(i);

                    obstacles_x
                        .entry(i.to_string())
                        .or_insert_with(Vec::new)
                        .push(n)
                }
                "^" => {
                    curr_position.x = i;
                    curr_position.y = n;
                }
                _ => continue,
            }
        }
    }

    for o in obstacles_y.values_mut() {
        o.sort();
    }

    for o in obstacles_x.values_mut() {
        o.sort();
    }

    let mut curr_dir = Directions::North;
    let mut obst_count = 0;
    let mut blockers: HashMap<Directions, Vec<Position>> = HashMap::new();
    while curr_position.x != 0
        && curr_position.x != max_pos.x
        && curr_position.y != 0
        && curr_position.y != max_pos.y
    {
        let step = check_step(
            &curr_dir,
            &curr_position,
            &obstacles_x,
            &obstacles_y,
            &max_pos,
        );

        blockers
            .entry(curr_dir.clone())
            .or_insert_with(Vec::new)
            .push(step);

        obst_count += 1;
        match curr_dir {
            Directions::North => match step.y {
                0 => {
                    curr_position.y = 0;
                }
                _ => {
                    curr_position.y = step.y;
                    curr_dir = Directions::East;
                }
            },
            Directions::South => match step.y {
                val if val == max_pos.y => {
                    curr_position.y = max_pos.y;
                }
                _ => {
                    curr_position.y = step.y;
                    curr_dir = Directions::West;
                }
            },
            Directions::East => match step.x {
                val if val == max_pos.x => {
                    curr_position.x = max_pos.x;
                }
                _ => {
                    curr_position.x = step.x;
                    curr_dir = Directions::South;
                }
            },
            Directions::West => match step.x {
                0 => {
                    curr_position.x = 0;
                }
                _ => {
                    curr_position.x = step.x;
                    curr_dir = Directions::North;
                }
            },
        }
    }

    let mut past_obst: HashMap<Directions, Vec<Position>> = HashMap::new();
    let mut dir = Directions::North;
    let mut sum = 0;
    for _ in 0..obst_count - 1 {
        let curr_obst = blockers.get_mut(&dir).unwrap().remove(0);

        // println!("{:?}", blockers);

        let next_dir = match dir {
            Directions::North => Directions::East,
            Directions::East => Directions::South,
            Directions::South => Directions::West,
            Directions::West => Directions::North,
        };
        let next_prom = blockers.get(&next_dir).unwrap().first();
        let next = match next_prom {
            Some(np) => np,
            None => continue,
        };
        let obstacles_to_bounce = past_obst.get(match next_dir {
            Directions::North => &Directions::East,
            Directions::East => &Directions::South,
            Directions::South => &Directions::West,
            Directions::West => &Directions::North,
        });

        if obstacles_to_bounce.is_some() {
            let otb: Vec<_> =obstacles_to_bounce 
                .unwrap()
                .iter()
                .filter(|val| match next_dir {
                    Directions::North => val.y < curr_obst.y && val.y > next.y,
                    Directions::East => val.x > curr_obst.x && val.x < next.x,
                    Directions::South => val.y > curr_obst.y && val.y < next.y,
                    Directions::West => val.x < curr_obst.x && val.x > next.x,
                })
                .collect();

            let bounce = otb.clone().len();
            println!("{:?}",otb );

            sum += bounce;
        }

        past_obst
            .entry(dir.clone())
            .or_insert_with(Vec::new)
            .push(curr_obst);

        dir = next_dir;
    }

    println!("{:?}", sum);
}

fn check_step(
    curr_dir: &Directions,
    curr_position: &Position,
    obstacles_x: &BTreeMap<String, Vec<usize>>,
    obstacles_y: &BTreeMap<String, Vec<usize>>,
    max_pos: &Position,
) -> Position {
    // Actually it's on his way but 'on my way' sounds better
    let obstacle_on_my_way = match curr_dir {
        Directions::North => {
            let po = obstacles_x[&curr_position.x.to_string()]
                .iter()
                .rev()
                .find(|&&y| y < curr_position.y);

            let new_y = match po {
                Some(y) => y + 1,
                None => 0,
            };
            Position {
                x: curr_position.x,
                y: new_y,
            }
        }
        Directions::South => {
            let po = obstacles_x[&curr_position.x.to_string()]
                .iter()
                .find(|&&y| y > curr_position.y);

            let new_y = match po {
                Some(y) => y - 1,
                None => max_pos.y,
            };

            Position {
                x: curr_position.x,
                y: new_y,
            }
        }
        Directions::East => {
            let po = obstacles_y[&curr_position.y.to_string()]
                .iter()
                .find(|&&x| x > curr_position.x);

            let new_x = match po {
                Some(x) => x - 1,
                None => max_pos.x,
            };

            Position {
                x: new_x,
                y: curr_position.y,
            }
        }
        Directions::West => {
            let po = obstacles_y[&curr_position.y.to_string()]
                .iter()
                .rev()
                .find(|&&x| x < curr_position.x);

            let new_x = match po {
                Some(x) => x + 1,
                None => 0,
            };

            Position {
                x: new_x,
                y: curr_position.y,
            }
        }
    };

    obstacle_on_my_way
}
