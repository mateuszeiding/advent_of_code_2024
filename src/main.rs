use utils::setup;
mod utils;

fn main() {
    let res = setup::get_input_lines_vec(String::from("./text.txt"));
    println!("Hello, world!");

    println!("Hello, world! {:?}", res);
}
