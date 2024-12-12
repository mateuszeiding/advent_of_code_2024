use crate::structs::position_struct::Position;
use crate::utils::setup;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(12, false);
    let matrix = setup::get_input_matrix(input);

    let mut plant_types: HashMap<&str, Vec<Position>> = HashMap::new();
    for y in 0..matrix.len() {
        matrix[y].iter().enumerate().for_each(|(x, plant)| {
            plant_types
                .entry(plant)
                .or_insert_with(Vec::new)
                .push(Position { x, y });
        });
    }

    let mut sum = 0;
    for (_, collection) in plant_types.iter() {
        let mut checked: Vec<Position> = Vec::new();
        let mut plants_to_check = vec![collection[0]];

        let mut fence_len = 0;
        let mut field_size = 0;
        while checked.len() < collection.len() {
            let unchecked: Vec<_> = plants_to_check
                .iter()
                .map(|ptc| {
                    field_size += 1;
                    let mut nbs = find_neighbros(&ptc, &collection);

                    fence_len += 4 - nbs.len();
                    let unchecked: Vec<Position> = nbs
                        .iter_mut()
                        .filter(|x| checked.iter().find(|prev| prev.eq(x)).is_none())
                        .map(|x| x.clone())
                        .collect();
                    return unchecked;
                })
                .collect();

            let mut unchecked_flat: Vec<_> = unchecked.iter().flatten().collect();

            unchecked_flat.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
            unchecked_flat.dedup();
            checked.extend(plants_to_check.clone());

            plants_to_check.clear();

            if unchecked_flat.len() == 0 {
                let next_plant = collection
                    .iter()
                    .find(|x| checked.iter().find(|prev| prev.eq(x)).is_none());

                sum += fence_len * field_size;
                fence_len = 0;
                field_size = 0;
                match next_plant {
                    None => continue,
                    Some(np) => plants_to_check.push(*np),
                }
            } else {
                plants_to_check.extend(unchecked_flat);
            }
        }
    }
    println!("{:?}", sum);
}

fn find_neighbros(curr_pos: &Position, plants: &Vec<Position>) -> Vec<Position> {
    let neighbros: Vec<Position> = plants
        .iter()
        .filter(|&p| {
            p.x.abs_diff(curr_pos.x) == 1 && p.y == curr_pos.y
                || p.y.abs_diff(curr_pos.y) == 1 && p.x == curr_pos.x
        })
        .map(|p| p.clone())
        .collect();

    return neighbros;
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(12, false);

    println!("{:#?}", input);
}
