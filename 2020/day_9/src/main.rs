mod encoding_error;
use encoding_error::{
    create_dataset_from,
    find_misbehaving_num_with_preamble,
};

fn main() {
    let dataset = create_dataset_from("test_data/input.txt");
    println!("The answer to part 1 is {}", find_misbehaving_num_with_preamble(25, &dataset));
    // answer is 248131121
}
