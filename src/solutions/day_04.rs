use crate::utils::setup;

#[allow(dead_code)]
pub fn part_01() {
    let input = setup::get_input_lines_vec(4, false);
    let matrix = setup::get_input_matrix(input);

    let mut x_position: Vec<(usize, usize)> = Vec::new();
    for n in 0..matrix.len() {
        let row = matrix[n].clone(); // Clone the row to avoid borrowing conflicts

        for (index, r) in row.iter().enumerate() {
            if r == "X" {
                x_position.push((index, n));
            }
        }
    }

    let mut occurences = 0;
    x_position.iter().for_each(|x| {
        occurences += xmas_checker(x.0, x.1, &matrix);
    });
    println!("{:#?}", occurences);
}

#[allow(dead_code)]
pub fn part_02() {
    let input = setup::get_input_lines_vec(4, false);

    let matrix = setup::get_input_matrix(input);

    let mut a_position: Vec<(usize, usize)> = Vec::new();
    for n in 0..matrix.len() {
        let row = matrix[n].clone(); // Clone the row to avoid borrowing conflicts

        for (index, r) in row.iter().enumerate() {
            if r == "A" {
                a_position.push((index, n));
            }
        }
    }

    let mut occurences = 0;
    a_position.iter().for_each(|x| {
        occurences += x_mas_checker(x.0, x.1, &matrix);
    });
    println!("{:#?}", occurences);
}

// Part 01 fn
// I know this solution is sad
fn xmas_checker(x: usize, y: usize, input: &Vec<Vec<String>>) -> usize {
    let (mut n, mut ne, mut nw, mut s, mut se, mut sw, mut e, mut w) = (0, 0, 0, 0, 0, 0, 0, 0);
    let search_val = ["X", "M", "A", "S"];

    for i in 0..4 {
        if y >= 3 {
            if n == i && input[y - i][x] == search_val[i] {
                n += 1;
            }

            if x >= 3 && nw == i && input[y - i][x - i] == search_val[i] {
                nw += 1;
            }

            if x + 3 < input[y - i].len() && ne == i && input[y - i][x + i] == search_val[i] {
                ne += 2;
            }
        }

        if y + 3 < input.len() {
            if s == i && input[y + i][x] == search_val[i] {
                s += 1;
            }

            if x >= 3 && sw == i && input[y + i][x - i] == search_val[i] {
                sw += 1;
            }

            if x + 3 < input[y + i].len() && se == i && input[y + i][x + i] == search_val[i] {
                println!("{} {} {}", "eeee", input[y + i][x + i], search_val[i]);
                se += 1;
            }
        }
        if x >= 3 && w == i && input[y][x - i] == search_val[i] {
            w += 1;
        }

        if x + 3 < input[y].len() && e == i && input[y][x + i] == search_val[i] {
            e += 1;
        }
    }

    let res = is_xmas(n)
        + is_xmas(ne)
        + is_xmas(nw)
        + is_xmas(s)
        + is_xmas(sw)
        + is_xmas(se)
        + is_xmas(w)
        + is_xmas(e);
    return res;
}

fn is_xmas(x: usize) -> usize {
    if x == 4 {
        1
    } else {
        0
    }
}

// Part 02 fn

fn x_mas_checker(x: usize, y: usize, input: &Vec<Vec<String>>) -> usize {
    if y == 0 || y + 1 >= input.len() || x == 0 || x + 1 >= input[y].len() {
        return 0;
    }

    let tl = input[y - 1][x - 1].clone();
    let tr = input[y - 1][x + 1].clone();

    let bl = input[y + 1][x - 1].clone();
    let br = input[y + 1][x + 1].clone();

    if !(tl == "M" && br == "S") && !(tl == "S" && br == "M") {
        return 0;
    }

    if !(tr == "M" && bl == "S") && !(tr == "S" && bl == "M") {
        return 0;
    }

    return 1;
}
