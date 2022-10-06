mod docking_data;
use docking_data::{
    register_input_from,
    execute_program,
};

fn main() {
    let input = register_input_from("test_data/input.txt");
    println!("The solution to part 1 is {}", execute_program(&input));
}
