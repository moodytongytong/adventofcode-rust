mod encoding_error;
use encoding_error::{
    create_dataset_from,
    find_misbehaving_num_with_preamble,
    find_weakness_with_target_sum,
};

fn main() {
    let dataset = create_dataset_from("test_data/input.txt");
    let part_1_answer = find_misbehaving_num_with_preamble(25, &dataset);
    println!("The answer to part 1 is {}", part_1_answer);
    println!("The answer to part 2 is {}", find_weakness_with_target_sum(part_1_answer, &dataset));
}