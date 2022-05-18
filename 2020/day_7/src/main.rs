mod handy_haversacks;

use handy_haversacks::{
    get_rules,
    find_num_of_colors_that_could_contain,
};

fn main() {
    let rules = get_rules("test_data/input.txt");
    println!("The answer to part 1 is {}", find_num_of_colors_that_could_contain("shiny gold", &rules));
}
