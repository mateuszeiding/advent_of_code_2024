use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::collections::BTreeMap;
use std::ops::Range;

enum Directions {
    North,
    East,
    South,
    West,
}

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(6, true);
    let matrix = setup::get_input_matrix(input);
    let mut curr_position: Position = Position { x: 0, y: 0 };
    let mut obstacles: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let max_pos: Position = Position {
        x: matrix[0].len(),
        y: matrix.len(),
    };


    for n in 0..matrix.len() {
        for (i, el) in matrix[n].iter().enumerate() {
            match el.as_str() {
                "#" => obstacles
                    .entry(n.to_string())
                    .or_insert_with(Vec::new)
                    .push(i),
                "^" => {
                    curr_position.x = i;
                    curr_position.y = n;
                }
                _ => continue,
            }
        }
    }

    println!("{:#?} {:?}", obstacles, curr_position);
    let mut curr_dir = Directions::North;
    let mut path: Vec<Range<usize>> = Vec::new();
    while curr_position.x == 0
        || curr_position.x == max_pos.x
        || curr_position.y == 0
        || curr_position.y == max_pos.y
    {
        match curr_dir {
            Directions::North => {}
            Directions::East => {}
            Directions::South => {}
            Directions::West => {}
        }
    }
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(6, false);

    println!("{:#?}", input);
}
