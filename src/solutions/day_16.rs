use crate::structs::maze_runner::MazeRunner;
use crate::utils::setup;
use std::rc::Rc;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(16, true);
    let matrix = setup::get_input_matrix(input);

    let mut mz = MazeRunner::new(Rc::new(matrix), "S", "E", ".");
    let winner = mz.run(false);
    println!("{:?}", winner[0].points);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(16, true);
    let matrix = setup::get_input_matrix(input);

    let mut mz = MazeRunner::new(Rc::new(matrix), "S", "E", ".");
    let winners = mz.run(true);
    let mut tiles: Vec<_> = winners.iter().map(|r| r.path.clone()).flatten().collect();
    tiles.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    tiles.dedup();

    // + 1 for E
    println!("{:?}", tiles.len() + 1);
}
