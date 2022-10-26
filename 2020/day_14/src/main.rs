mod docking_data;
use docking_data::{
    register_input_from,
    execute_program1,
    execute_program2,
};

fn main() {
    let input = register_input_from("test_data/input.txt");
    println!("The solution to part 1 is {}", execute_program1(&input));
    println!("The solution to part 2 is {}", execute_program2(&input));
}
