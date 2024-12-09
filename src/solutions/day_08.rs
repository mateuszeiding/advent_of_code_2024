use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(8, false);
    let matrix = setup::get_input_matrix(input);

    let mut antenas: HashMap<String, Vec<Position>> = HashMap::new();
    matrix.iter().enumerate().for_each(|(i, x)| {
        x.iter().enumerate().for_each(|(j, y)| match y.as_str() {
            "." => {}
            _ => {
                antenas
                    .entry(y.to_string())
                    .or_insert_with(Vec::new)
                    .push(Position { x: j, y: i });
            }
        });
    });

    let max_x = matrix[0].len() as i32;
    let max_y = matrix.len() as i32;
    let mut antinodes: Vec<Position> = Vec::new();
    for ant in antenas.values() {
        for n in 0..ant.len() {
            for m in n + 1..ant.len() {
                get_antinodes(&ant[n], &ant[m], &mut antinodes, max_x, max_y);
            }
        }
    }
    antinodes.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    antinodes.dedup();
    println!("{:#?}", antinodes);
    println!("{}", antinodes.len());
}

fn get_antinodes(
    ant_1: &Position,
    ant_2: &Position,
    antinodes: &mut Vec<Position>,
    max_x: i32,
    max_y: i32,
) {
    let x_diff: i32 = ant_1.x as i32 - ant_2.x as i32;
    let y_diff: i32 = ant_1.y as i32 - ant_2.y as i32;
    let node_1_x;
    let node_2_x;
    let node_1_y;
    let node_2_y;
    if x_diff < 0 {
        node_1_x = ant_2.x as i32 + x_diff.abs();
        node_2_x = ant_1.x as i32 - x_diff.abs();
        if y_diff < 0 {
            node_1_y = ant_2.y as i32 + y_diff.abs();
            node_2_y = ant_1.y as i32 - y_diff.abs();
        } else {
            node_1_y = ant_1.y as i32 + y_diff.abs();
            node_2_y = ant_2.y as i32 - y_diff.abs();
        }
    } else {
        node_1_x = ant_1.x as i32 + x_diff.abs();
        node_2_x = ant_2.x as i32 - x_diff.abs();
        if y_diff < 0 {
            node_1_y = ant_1.y as i32 - y_diff.abs();
            node_2_y = ant_2.y as i32 + y_diff.abs();
        } else {
            node_1_y = ant_2.y as i32 - y_diff.abs();
            node_2_y = ant_1.y as i32 + y_diff.abs();
        }
    };

    if node_1_y >= 0 && node_1_y < max_y && node_1_x >= 0 && node_1_x < max_x {
        antinodes.push(Position {
            x: node_1_x as usize,
            y: node_1_y as usize,
        });
    }

    if node_2_y >= 0 && node_2_y < max_y && node_2_x >= 0 && node_2_x < max_x {
        antinodes.push(Position {
            x: node_2_x as usize,
            y: node_2_y as usize,
        });
    }
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(8, false);
    let matrix = setup::get_input_matrix(input);

    let mut antenas: HashMap<String, Vec<Position>> = HashMap::new();
    matrix.iter().enumerate().for_each(|(i, x)| {
        x.iter().enumerate().for_each(|(j, y)| match y.as_str() {
            "." => {}
            _ => {
                antenas
                    .entry(y.to_string())
                    .or_insert_with(Vec::new)
                    .push(Position { x: j, y: i });
            }
        });
    });

    let max_x = matrix[0].len() as i32;
    let max_y = matrix.len() as i32;
    let mut antinodes: Vec<Position> = Vec::new();
    for ant in antenas.values() {
        for n in 0..ant.len() {
            if ant.len() > 1 {
                antinodes.push(Position {
                    x: ant[n].x,
                    y: ant[n].y,
                });
            }
            for m in n + 1..ant.len() {
                get_antinodes_02(&ant[n], &ant[m], &mut antinodes, max_x, max_y);
            }
        }
    }

    antinodes.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    antinodes.dedup();
    println!("{}", antinodes.len());
}

fn get_antinodes_02(
    ant_1: &Position,
    ant_2: &Position,
    antinodes: &mut Vec<Position>,
    max_x: i32,
    max_y: i32,
) {
    let x_diff: i32 = ant_1.x as i32 - ant_2.x as i32;
    let y_diff: i32 = ant_1.y as i32 - ant_2.y as i32;
    let mut node_1_x;
    let mut node_2_x;
    let mut node_1_y;
    let mut node_2_y;
    if x_diff < 0 {
        node_1_x = ant_2.x as i32 + x_diff.abs();
        node_2_x = ant_1.x as i32 - x_diff.abs();
        if y_diff < 0 {
            node_1_y = ant_2.y as i32 + y_diff.abs();
            node_2_y = ant_1.y as i32 - y_diff.abs();
        } else {
            node_1_y = ant_1.y as i32 + y_diff.abs();
            node_2_y = ant_2.y as i32 - y_diff.abs();
        }
    } else {
        node_1_x = ant_1.x as i32 + x_diff.abs();
        node_2_x = ant_2.x as i32 - x_diff.abs();
        if y_diff < 0 {
            node_1_y = ant_1.y as i32 - y_diff.abs();
            node_2_y = ant_2.y as i32 + y_diff.abs();
        } else {
            node_1_y = ant_2.y as i32 - y_diff.abs();
            node_2_y = ant_1.y as i32 + y_diff.abs();
        }
    };

    while node_1_y >= 0 && node_1_y < max_y && node_1_x >= 0 && node_1_x < max_x {
        antinodes.push(Position {
            x: node_1_x as usize,
            y: node_1_y as usize,
        });
        if x_diff < 0 {
            node_1_x += x_diff.abs();
            if y_diff < 0 {
                node_1_y += y_diff.abs();
            } else {
                node_1_y += y_diff.abs();
            }
        } else {
            node_1_x += x_diff.abs();
            if y_diff < 0 {
                node_1_y -= y_diff.abs();
            } else {
                node_1_y -= y_diff.abs();
            }
        };
    }

    while node_2_y >= 0 && node_2_y < max_y && node_2_x >= 0 && node_2_x < max_x {
        antinodes.push(Position {
            x: node_2_x as usize,
            y: node_2_y as usize,
        });
        if x_diff < 0 {
            node_2_x -= x_diff.abs();
            if y_diff < 0 {
                node_2_y -= y_diff.abs();
            } else {
                node_2_y -= y_diff.abs();
            }
        } else {
            node_2_x -= x_diff.abs();
            if y_diff < 0 {
                node_2_y += y_diff.abs();
            } else {
                node_2_y += y_diff.abs();
            }
        };
    }
}
