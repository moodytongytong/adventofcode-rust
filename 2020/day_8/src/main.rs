mod handheld_halting;
use handheld_halting::{
    GameConsole
};

fn main() {
    let mut console = GameConsole::new("test_data/input.txt");
    println!("The answer to part 1 is {}", console.find_accumulator_before_loop());
    println!("The answer to part 2 is {}", console.accumulator_value_when_terminating_normally());
}
