use crate::structs::maze_runner::MazeRunner;
use crate::utils::setup;
use std::rc::Rc;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(16, false);
    let matrix = setup::get_input_matrix(input);

    let mut mz = MazeRunner::new(Rc::new(matrix), "S", "E", ".");
    let winner = mz.run(false);
    println!("{:?}", winner);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(16, true);

    println!("{:#?}", input);
}
