mod adapter_array;
use adapter_array::{
<<<<<<< HEAD
    create_ascending_adapters,
    find_differences_of_1_and_3_jolts_from,
    find_number_of_arrangements,
};

fn main() {
    let adapters = create_ascending_adapters("test_data/input.txt");
    let differences = find_differences_of_1_and_3_jolts_from(&adapters);
    println!("The product of the two differences is {}", differences.1 * differences.0);
    println!("The total number of arrangements is {}", find_number_of_arrangements(&adapters));
=======
    create_adapters,
    find_differences_of_1_and_3_jolts_from,
};

fn main() {
    let adapters = create_adapters("test_data/input.txt");
    let differences = find_differences_of_1_and_3_jolts_from(adapters);
    println!("The product of the two differences is {}", differences.1 * differences.0);
>>>>>>> 4b9832353afd09eee2a50777a9e6aab20fa22b56
}
