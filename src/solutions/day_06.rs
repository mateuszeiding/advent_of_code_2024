use crate::enums::direction_enum::Direction;
use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;

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
    let mut curr_dir = Direction::North;
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
            Direction::North => {
                let obstacle = obstacles_x[&curr_position.x.to_string()]
                    .iter()
                    .rev()
                    .find(|&&y| y < curr_position.y);

                match obstacle {
                    Some(o) => {
                        map_y_ranges(&mut path, *o + 1..curr_position.y, curr_position.x);
                        curr_position.y = o + 1;
                        curr_dir = Direction::East;
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
            Direction::South => {
                let obstacle = obstacles_x[&curr_position.x.to_string()]
                    .iter()
                    .find(|&&y| y > curr_position.y);

                match obstacle {
                    Some(o) => {
                        map_y_ranges(&mut path, curr_position.y..*o, curr_position.x);

                        curr_position.y = o - 1;
                        curr_dir = Direction::West;
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
            Direction::East => {
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
                        curr_dir = Direction::South;
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
            Direction::West => {
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
                        curr_dir = Direction::North;
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
    let mut curr_position: Position = Position::new(0, 0);
    let mut obstacles: Vec<Position> = Vec::new();
    let max_pos: Position = Position::new(matrix[0].len(), matrix.len());

    for y in 0..matrix.len() {
        for (x, el) in matrix[y].iter().enumerate() {
            match el.as_str() {
                "#" => obstacles.push(Position::new(x, y)),
                "^" => {
                    curr_position.x = x;
                    curr_position.y = y;
                }
                _ => continue,
            }
        }
    }

    obstacles.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));

    let finder = |obst: &&Position, cp: &Position, dir: &Direction| match dir {
        Direction::North => obst.x == cp.x && obst.y < cp.y,
        Direction::South => obst.x == cp.x && obst.y > cp.y,
        Direction::East => obst.y == cp.y && obst.x > cp.x,
        Direction::West => obst.y == cp.y && obst.x < cp.x,
    };

    let mut past_obst: HashMap<Direction, Vec<Position>> = HashMap::new();
    let go_on_adventure = |scp: &Position,
                           sdir: &Direction,
                           po: &HashMap<Direction, Vec<Position>>|
     -> Option<(Direction, Position)> {
        let mut cp = scp.clone();
        let mut dir = sdir.clone().next();
        let mut matrix_cp = matrix.clone();
        let print_obstacle = |matrix_cp: &mut Vec<Vec<String>>, x: usize, y: usize| {
            matrix_cp[y][x] = "•".to_string();
            setup::print_matrix(
                &matrix_cp,
                Some(|_y, _x, c: String| {
                    print!("{}", c);
                }),
            );
        };

        while !cp.is_min_max(max_pos.x, max_pos.y) {
            match obstacles.iter().rev().find(|x| finder(x, &cp, &dir)) {
                None => match dir {
                    Direction::North => cp.y = 0,
                    Direction::South => cp.y = max_pos.y,
                    Direction::East => cp.x = max_pos.x,
                    Direction::West => cp.x = 0,
                },
                Some(obst) => {
                    let po_vec = po.get(&dir);
                    if po_vec.is_some() && po_vec.unwrap().iter().find(|&x| x.eq(obst)).is_some() {
                        cp.next_step(&dir);
                        dir = dir.next();
                        print_obstacle(&mut matrix_cp, cp.x, cp.y);

                        return Some((dir, *obst));
                    }

                    matrix_cp[cp.y][cp.x] = "0".to_string();
                    dir = dir.next();
                }
            }
        }

        None
    };

    let mut sum = 0;
    let mut curr_dir = Direction::North;

    let mut umpaloop_village: HashMap<Direction, Vec<Position>> = HashMap::new();

    while curr_position.is_not_min_max(max_pos.x, max_pos.y) {
        let going_there = match obstacles
            .iter()
            .find(|obst| finder(obst, &curr_position, &curr_dir))
        {
            None => match curr_dir {
                Direction::North => Position::new(curr_position.x, 0),
                Direction::South => Position::new(curr_position.x, max_pos.y + 1),
                Direction::East => Position::new(max_pos.x + 1, curr_position.y),
                Direction::West => Position::new(0, curr_position.y),
            },
            Some(ob) => *ob,
        };

        past_obst
            .entry(curr_dir.clone())
            .or_insert_with(Vec::new)
            .push(going_there);

        while match curr_dir {
            Direction::North => curr_position.y > going_there.y + 1,
            Direction::South => curr_position.y < going_there.y - 1,
            Direction::East => curr_position.x < going_there.x - 1,
            Direction::West => curr_position.x > going_there.x + 1,
        } {
            curr_position.next_step(&curr_dir);
            match go_on_adventure(&curr_position, &curr_dir, &past_obst) {
                Some(lu) => {
                    let (k, v) = lu;
                    println!("{:?}", lu.1.clone());
                    umpaloop_village.entry(k).or_insert_with(Vec::new).push(v);
                    sum += 1;
                }
                None => {}
            }

            if curr_position.is_min_max(max_pos.x, max_pos.y) {
                break;
            }
        }

        curr_dir = curr_dir.next();
    }

    umpaloop_village.iter_mut().for_each(|(_, v)| {
        v.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
        v.dedup();
    });

    println!(
        "{:?}",
        umpaloop_village.values().map(|v| v.len()).sum::<usize>()
    );

    let mut m: Vec<_> = umpaloop_village
        .iter_mut()
        .flat_map(|(_, v)| v.clone())
        .collect();

    m.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    m.dedup();
    println!("{:?} {:}", m, m.len());
    println!("{:?}", sum);
}
