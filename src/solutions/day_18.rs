use crate::structs::maze_runner::MazeRunner;
use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::rc::Rc;

#[allow(dead_code)]
pub fn part_01() {
    let test = false;
    let bytes_to_load = if test { 12 } else { 1024 };
    let input = setup::get_input_lines_vec(18, test);

    let corrupt_bytes: Vec<Position> = input[..bytes_to_load]
        .iter()
        .map(|p| {
            let split: Vec<usize> = p.split(",").map(|p| p.parse::<usize>().unwrap()).collect();

            Position {
                x: split[0],
                y: split[1],
            }
        })
        .collect();

    let mut matrix: Vec<Vec<String>> = Vec::new();
    println!("{:?}", corrupt_bytes);
    let max = if test { 7 } else { 71 };
    let max_pos = Position { x: max, y: max };

    (0..=max_pos.y + 1).for_each(|y| {
        matrix.push(Vec::new());
        (0..=max_pos.x + 1).for_each(|x| {
            if x == 1 && y == 1 {
                matrix[y].push("S".to_string());
            } else if x == max_pos.x && y == max_pos.y {
                matrix[y].push("E".to_string());
            } else if corrupt_bytes
                .iter()
                .find(|cb| cb.x + 1 == x && cb.y + 1 == y)
                .is_some()
                || x == 0
                || x == max_pos.x + 1
                || y == 0
                || y == max_pos.y + 1
            {
                matrix[y].push("#".to_string());
            } else {
                matrix[y].push(".".to_string());
            }
        });

        println!();
    });

    let mut mz = MazeRunner::new(Rc::new(matrix.clone()), "S", "E", ".");
    let winner = mz.shortest(true);
    println!(
        "{:?}",
        setup::print_matrix::<fn(usize, usize, String)>(&matrix, None)
    );
    println!("{:?}", winner.path.len());
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(18, false);

    println!("{:#?}", input);
}
