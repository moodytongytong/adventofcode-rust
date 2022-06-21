mod adapter_array;
use adapter_array::{
    create_adapters,
    find_differences_of_1_and_3_jolts_from,
};

fn main() {
    let adapters = create_adapters("test_data/input.txt");
    let differences = find_differences_of_1_and_3_jolts_from(adapters);
    println!("The product of the two differences is {}", differences.1 * differences.0);
}
