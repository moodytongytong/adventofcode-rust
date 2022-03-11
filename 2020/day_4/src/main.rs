mod passport_processing;
use passport_processing::{
    create_records_from_path,
    find_number_of_possible_passports,
    find_number_of_valid_passports
};
use std::io;

fn main() -> Result<(), io::Error> {
    let records = create_records_from_path("test_data/input.txt")?;
    println!("The number of possible passports is {}", find_number_of_possible_passports(&records));
    println!("The number of valid passports is {}", find_number_of_valid_passports(&records));
    Ok(())
}
