mod ticket_translation;
use ticket_translation::create_formatted_input_from;

fn main() {
    let input = create_formatted_input_from("test_data/input.txt");
    println!("The answer to part 1 is {}", input.find_sum_of_invalid_values());
}
