use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;

#[derive(Debug, Clone)]
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

    let mut curr_dir = Directions::North;
    let mut sum = 0;
    let mut blockers: Vec<Position> = Vec::new();
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
                        for n in (o + 1..curr_position.y).rev() {
                            let res = check_next_three(
                                Directions::East,
                                curr_position.x,
                                n,
                                &obstacles_x,
                                &obstacles_y,
                            );
                            if res == 1 {
                                sum += 1;
                                let exist = blockers.iter().position(|b| {
                                    b.x == curr_position.x - 1 && b.y == curr_position.y
                                });
                                if exist.is_none() {
                                    blockers.push(Position {
                                        y: curr_position.y,
                                        x: curr_position.x - 1,
                                    });
                                }
                                break;
                            }
                        }
                        curr_position.y = o + 1;
                        curr_dir = Directions::East;
                    }
                    None => {
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
                        for n in curr_position.y..o - 1 {
                            let res = check_next_three(
                                Directions::West,
                                curr_position.x,
                                n,
                                &obstacles_x,
                                &obstacles_y,
                            );
                            if res == 1 {
                                sum += 1;
                                let exist = blockers.iter().position(|b| {
                                    b.x == curr_position.x + 1 && b.y == curr_position.y
                                });
                                if exist.is_none() {
                                    blockers.push(Position {
                                        y: curr_position.y,
                                        x: curr_position.x + 1,
                                    });
                                }
                                break;
                            }
                        }
                        curr_position.y = o - 1;
                        curr_dir = Directions::West;
                    }
                    None => {
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
                        for n in curr_position.x..o - 1 {
                            let res = check_next_three(
                                Directions::South,
                                n,
                                curr_position.y,
                                &obstacles_x,
                                &obstacles_y,
                            );
                            if res == 1 {
                                sum += 1;
                                let exist = blockers.iter().position(|b| {
                                    b.y == curr_position.y - 1 && b.x == curr_position.x
                                });
                                if exist.is_none() {
                                    blockers.push(Position {
                                        x: curr_position.x,
                                        y: curr_position.y,
                                    });
                                }
                                break;
                            }
                        }
                        curr_position.x = o - 1;
                        curr_dir = Directions::South;
                    }
                    None => {
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
                        for n in (o + 1..curr_position.x).rev() {
                            let res = check_next_three(
                                Directions::North,
                                n,
                                curr_position.y,
                                &obstacles_x,
                                &obstacles_y,
                            );
                            if res == 1 {
                                sum += 1;
                                let exist = blockers.iter().position(|b| {
                                    b.y == curr_position.y + 1 && b.x == curr_position.x
                                });
                                if exist.is_none() {
                                    blockers.push(Position {
                                        x: curr_position.x,
                                        y: curr_position.y,
                                    });
                                }
                                break;
                            }
                        }
                        curr_position.x = o + 1;
                        curr_dir = Directions::North;
                    }
                    None => {
                        curr_position.x = 0;
                    }
                }
            }
        }
    }
    println!("{:?} {} {:?}", "-----", sum, curr_position);
    println!("{:?} ", blockers);
    println!("{:?}", blockers.len());
}

fn check_next_three(
    curr_dir: Directions,
    x: usize,
    y: usize,
    hm_x: &BTreeMap<String, Vec<usize>>,
    hm_y: &BTreeMap<String, Vec<usize>>,
) -> usize {
    let mut dir = curr_dir.clone();
    let mut pos = Position { x, y };
    let mut res = 0;

    for _ in 0..4 {
        match dir {
            Directions::North => {
                if !hm_x.contains_key(&pos.x.to_string()) {
                    break;
                }
                let check = check_one_step(&dir, &pos, &hm_x, &hm_y);
                dir = Directions::East;
            }

            Directions::South => {
                if !hm_x.contains_key(&pos.x.to_string()) {
                    break;
                }
                let check = check_one_step(&dir, &pos, &hm_x, &hm_y);
                dir = Directions::West;
            }

            Directions::West => {
                if !hm_y.contains_key(&pos.y.to_string()) {
                    break;
                }
                let check = check_one_step(&dir, &pos, &hm_x, &hm_y);
                dir = Directions::North;
            }
            Directions::East => {
                if !hm_y.contains_key(&pos.y.to_string()) {
                    break;
                }
                let check = check_one_step(&dir, &pos, &hm_x, &hm_y);
                dir = Directions::South;
            }
        };
        res += 1;
    }
    if res == 4 {
        1
    } else {
        0
    }
}
fn check_one_step(
    curr_dir: &Directions,
    curr_position: &Position,
    obstacles_x: &BTreeMap<String, Vec<usize>>,
    obstacles_y: &BTreeMap<String, Vec<usize>>,
) -> (Directions, Position) {
    // Actually it's on his way but 'on my way' sounds better
    let obstacle_on_my_way = match curr_dir {
        Directions::North => {
            let po = obstacles_x[&curr_position.x.to_string()]
                .iter()
                .rev()
                .find(|&&y| y < curr_position.y);

            Position {
                x: curr_position.x,
                y: po.unwrap_or(&0).clone(),
            }
        }
        Directions::South => {
            let po = obstacles_x[&curr_position.x.to_string()]
                .iter()
                .find(|&&y| y > curr_position.y);

            Position {
                x: curr_position.x,
                y: po.unwrap_or(&10).clone(),
            }
        }
        Directions::East => {
            let po = obstacles_y[&curr_position.y.to_string()]
                .iter()
                .find(|&&x| x > curr_position.x);

            Position {
                x: po.unwrap_or(&10).clone(),
                y: curr_position.y,
            }
        }
        Directions::West => {
            let po = obstacles_y[&curr_position.y.to_string()]
                .iter()
                .rev()
                .find(|&&x| x < curr_position.x);

            Position {
                x: po.unwrap_or(&0).clone(),
                y: curr_position.x,
            }
        }
    };

    return (curr_dir.clone(), obstacle_on_my_way);
}
