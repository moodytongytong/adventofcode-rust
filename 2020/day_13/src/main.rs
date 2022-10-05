mod shuttle_search;
use shuttle_search::find_wait_and_bus_id_product_from;
use shuttle_search::create_input_holder;
use shuttle_search::the_earliest_time;

fn main() {
    let input = create_input_holder("test_data/input.txt");
    println!("The solution to part 1 is {}", find_wait_and_bus_id_product_from(&input));
    println!("The solution to part 2 is {}", the_earliest_time(&input[1])); 
}
