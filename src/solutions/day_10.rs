use crate::structs::position_struct::Position;
use crate::utils::setup;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(10, false);
    let matrix = setup::get_input_matrix(input)
        .iter()
        .map(|x| x.iter().map(|y| y.parse::<usize>().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();

    let max_pos = Position {
        x: matrix[0].len() - 1,
        y: matrix.len() - 1,
    };

    let mut all_heads: Vec<Position> = Vec::new();
    matrix.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &column)| {
            if column == 0 {
                all_heads.push(Position { x, y });
            }
        })
    });

    let mut im_here: Vec<Vec<Position>> = Vec::new();
    for i in 0..all_heads.len() {
        im_here.push(vec![all_heads[i]]);
        for n in 0..9 {
            let mut ill_be_there: Vec<Position> = Vec::new();
            im_here[i].iter().for_each(|here| {
                let new_pos = look_around(&here, &n, &matrix, &max_pos);
                match new_pos {
                    Some(np) => ill_be_there.extend(np),
                    None => {}
                }
            });

            im_here[i] = ill_be_there;
        }
    }
    im_here.iter_mut().for_each(|x| {
        x.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
        x.dedup();
    });
    let all_tops: Vec<_> = im_here.iter().flatten().collect();
    println!("{:#?}", all_tops.len());
}

fn look_around(
    position: &Position,
    curr_height: &usize,
    map: &Vec<Vec<usize>>,
    max_pos: &Position,
) -> Option<Vec<Position>> {
    let mut can_go_there: Vec<Position> = Vec::new();
    if position.y > 0 {
        let up = map[position.y - 1][position.x];
        if curr_height + 1 == up {
            can_go_there.push(Position {
                x: position.x,
                y: position.y - 1,
            });
        }
    }

    if position.y < max_pos.y {
        let down = map[position.y + 1][position.x];
        if curr_height + 1 == down {
            can_go_there.push(Position {
                x: position.x,
                y: position.y + 1,
            });
        }
    }

    if position.x > 0 {
        let left = map[position.y][position.x - 1];
        if curr_height + 1 == left {
            can_go_there.push(Position {
                x: position.x - 1,
                y: position.y,
            });
        }
    }

    if position.x < max_pos.x {
        let right = map[position.y][position.x + 1];
        if curr_height + 1 == right {
            can_go_there.push(Position {
                x: position.x + 1,
                y: position.y,
            });
        }
    }

    return match can_go_there.len() == 0 {
        true => None,
        false => Some(can_go_there),
    };
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(10, false);
    let matrix = setup::get_input_matrix(input)
        .iter()
        .map(|x| x.iter().map(|y| y.parse::<usize>().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();

    let max_pos = Position {
        x: matrix[0].len() - 1,
        y: matrix.len() - 1,
    };

    let mut all_heads: Vec<Position> = Vec::new();
    matrix.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &column)| {
            if column == 0 {
                all_heads.push(Position { x, y });
            }
        })
    });

    let mut im_here: Vec<Vec<Position>> = Vec::new();
    for i in 0..all_heads.len() {
        im_here.push(vec![all_heads[i]]);
        for n in 0..9 {
            let mut ill_be_there: Vec<Position> = Vec::new();
            im_here[i].iter().for_each(|here| {
                let new_pos = look_around(&here, &n, &matrix, &max_pos);
                match new_pos {
                    Some(np) => ill_be_there.extend(np),
                    None => {}
                }
            });

            im_here[i] = ill_be_there;
        }
    }
    let all_tops: Vec<_> = im_here.iter().flatten().collect();
    println!("{:#?}", all_tops.len());
}
