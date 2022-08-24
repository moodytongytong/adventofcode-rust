mod shuttle_search;
use shuttle_search::find_product_from;

fn main() {
    println!("The solution to part 1 is {}", find_product_from("test_data/input.txt"));
}
