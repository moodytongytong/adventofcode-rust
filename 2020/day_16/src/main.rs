mod sum_finder;
use sum_finder::Finder;

fn main() {
    let finder = Finder::new("./test_data/input.txt");
    println!("Answer to part 1 is {}", finder.find_product_of_two_targets_sum_to(2020));
    println!("Answer to part 2 is {}", finder.find_product_of_three_targets_sum_to(2020));

    // could refactor the error handling part to prettify the code
}
