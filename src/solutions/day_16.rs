use crate::structs::maze_runner::MazeRunner;
use crate::utils::setup;
use std::rc::Rc;
use std::time::Instant;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(16, false);
    let matrix = setup::get_input_matrix(input);

    let mut mz = MazeRunner::new(Rc::new(matrix), "S", "E", ".");
    let winner = mz.run(false);
    println!("{:?}", winner[0].points);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(16, false);
    let matrix = setup::get_input_matrix(input);

    let start = Instant::now();
    let mut mz = MazeRunner::new(Rc::new(matrix.clone()), "S", "E", ".");
    let winners = mz.run(false);
    let mut tiles: Vec<_> = winners.iter().map(|r| r.path.clone()).flatten().collect();
    tiles.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    tiles.dedup();

    let dur = start.elapsed();
    println!("dur {:?} ", dur);
    setup::print_matrix(
        &matrix,
        Some(|y, x, c: String| {
            if tiles.iter().find(|p| p.x == x && p.y == y).is_some() {
                print!("•");
            } else {
                print!("{}", c);
            }
        }),
    );

    println!("{:?}", tiles.len() + 1);
}
