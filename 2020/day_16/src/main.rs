mod ticket_translation;
use ticket_translation::create_formatted_input_from;

fn main() {
    let input = create_formatted_input_from("test_data/input.txt");
    println!("The answer to part 1 is {}", input.find_sum_of_invalid_values());

    input.populate_field_position_map();
    let mut part2_solution = 1;
    for (index, category) in input.field_position.borrow().iter() {
        if category.contains("departure") {
            part2_solution *= input.my_ticket[*index];
        }
    }
    println!("The solution to part 2 is {}", part2_solution);
}
