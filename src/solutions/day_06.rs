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

impl Directions {
    fn next(self) -> Directions {
        match self {
            Directions::North => Directions::East,
            Directions::East => Directions::South,
            Directions::South => Directions::West,
            Directions::West => Directions::North,
        }
    }
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
    let mut obstacles: Vec<Position> = Vec::new();
    let max_pos: Position = Position {
        x: matrix[0].len().clone() - 1,
        y: matrix.len().clone() - 1,
    };

    for y in 0..matrix.len() {
        for (x, el) in matrix[y].iter().enumerate() {
            match el.as_str() {
                "#" => obstacles.push(Position { x, y }),
                "^" => {
                    curr_position.x = x;
                    curr_position.y = y;
                }
                _ => continue,
            }
        }
    }

    obstacles.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));

    let finder = |obst: &&Position, cp: &Position, dir: &Directions| match dir {
        Directions::North => obst.x == cp.x && obst.y < cp.y,
        Directions::South => obst.x == cp.x && obst.y > cp.y,
        Directions::East => obst.y == cp.y && obst.x > cp.x,
        Directions::West => obst.y == cp.y && obst.x < cp.x,
    };

    let mut past_obst: HashMap<Directions, Vec<Position>> = HashMap::new();
    let go_on_adventure = |scp: &Position,
                           sdir: &Directions,
                           po: &HashMap<Directions, Vec<Position>>|
     -> Option<Position> {
        let mut cp = scp.clone();
        let mut dir = sdir.clone().next();

        while cp.x != 0 && cp.x != max_pos.x && cp.y != 0 && cp.y != max_pos.y {
            let obstacle = obstacles.iter().find(|x| finder(x, &cp, &dir));

            match obstacle {
                None => match dir {
                    Directions::North => cp.y = 0,
                    Directions::South => cp.y = max_pos.y,
                    Directions::East => cp.x = max_pos.x,
                    Directions::West => cp.x = 0,
                },

                Some(obst) => {
                    let po_vec = po.get(&dir);
                    if po_vec.is_some() && po_vec.unwrap().iter().find(|&x| x.eq(obst)).is_some() {
                        return Some(cp.clone());
                    }

                    match dir {
                        Directions::North => cp.y = obst.y + 1,
                        Directions::South => cp.y = obst.y - 1,
                        Directions::East => cp.x = obst.x - 1,
                        Directions::West => cp.x = obst.x + 1,
                    }

                    dir = dir.next();
                }
            }
        }
        return None;
    };

    let mut sum = 0;
    let mut curr_dir = Directions::North;

    let mut umpaloop_village: Vec<Position> = Vec::new();
    while curr_position.x != 0
        && curr_position.x != max_pos.x
        && curr_position.y != 0
        && curr_position.y != max_pos.y
    {
        let obstacle = obstacles
            .iter()
            .find(|x| finder(x, &curr_position, &curr_dir));

        let mut addi = 0;
        let going_there = if obstacle.is_none() {
            addi = 1;
            match curr_dir {
                Directions::North => Position {
                    y: 0,
                    x: curr_position.x,
                },
                Directions::South => Position {
                    y: max_pos.y,
                    x: curr_position.x,
                },
                Directions::East => Position {
                    x: max_pos.x,
                    y: curr_position.y,
                },
                Directions::West => Position {
                    x: 0,
                    y: curr_position.y,
                },
            }
        } else {
            *obstacle.unwrap()
        };

        past_obst
            .entry(curr_dir.clone())
            .or_insert_with(Vec::new)
            .push(going_there);

        while match curr_dir {
            Directions::North => curr_position.y > going_there.y + 1 - addi,
            Directions::South => curr_position.y < going_there.y - 1 + addi,

            Directions::East => curr_position.x < going_there.x - 1 + addi,
            Directions::West => curr_position.x > going_there.x + 1 - addi,
        } {
            match curr_dir {
                Directions::North => curr_position.y -= 1,
                Directions::South => curr_position.y += 1,
                Directions::East => curr_position.x += 1,
                Directions::West => curr_position.x -= 1,
            }
            //println!("{:?} {:?} {:?}", curr_position, going_there, curr_dir);

            let loopa_umper = go_on_adventure(&curr_position, &curr_dir, &past_obst);
            if loopa_umper.is_some() {
                let lu = loopa_umper.unwrap();
                if umpaloop_village.iter().find(|x| x.eq(&&lu)).is_none() {
                    umpaloop_village.push(loopa_umper.unwrap());
                    sum += 1;
                }
            }
            if curr_position.x == 0
                || curr_position.x == max_pos.x
                || curr_position.y == 0
                || curr_position.y == max_pos.y
            {
                break;
            }
        }
        curr_dir = curr_dir.next();
    }

    println!("{:?}", umpaloop_village);
    println!("{:?}", sum);
}
